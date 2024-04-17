use bevy::prelude::*;
use bevy_rapier2d::plugin::{NoUserData, RapierPhysicsPlugin};
use game::{ball_position, text_update_system, BallCoords};

mod game;

pub fn init_bevy_game() {
    let mut bevy_app = App::new();

    #[allow(unused_mut)]
    let mut default_plugins = DefaultPlugins.build();

    // Temporary fix for the crash caused by winit on macOS Sonoma.
    #[cfg(target_os = "macos")]
    {
        default_plugins = default_plugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: bevy::window::WindowResolution::new(900., 700.)
                    .with_scale_factor_override(2.0),
                resize_constraints: WindowResizeConstraints {
                    min_width: 600.,
                    min_height: 500.,
                    max_height: 1200.,
                    max_width: 1600.,
                },
                ..default()
            }),
            #[cfg(target_os = "android")]
            close_when_requested: false,
            ..default()
        });
    }

    #[cfg(any(target_os = "android", target_os = "ios"))]
    {
        default_plugins = default_plugins
            .disable::<WinitPlugin>()
            .set(WindowPlugin::default());
    }

    #[cfg(target_os = "android")]
    {
        bevy_app.insert_non_send_resource(android_asset_manager);

        use bevy::render::{
            settings::{RenderCreation, WgpuSettings},
            RenderPlugin,
        };
        default_plugins = default_plugins.set(RenderPlugin {
            render_creation: RenderCreation::Automatic(WgpuSettings {
                backends: Some(wgpu::Backends::VULKAN),
                ..default()
            }),
        });

        // the custom asset io plugin must be inserted in-between the
        // `CorePlugin' and `AssetPlugin`. It needs to be after the
        // CorePlugin, so that the IO task pool has already been constructed.
        // And it must be before the `AssetPlugin` so that the asset plugin
        // doesn't create another instance of an asset server. In general,
        // the AssetPlugin should still run so that other aspects of the
        // asset system are initialized correctly.
        //
        // 2023/11/04, Bevy v0.12:
        // In the Android, Bevy's AssetPlugin relies on winit, which we are not using.
        // If a custom AssetPlugin plugin is not provided,  it will crash at runtime:
        // thread '<unnamed>' panicked at 'Bevy must be setup with the #[bevy_main] macro on Android'
        default_plugins = default_plugins
            .add_before::<bevy::asset::AssetPlugin, _>(android_asset_io::AndroidAssetIoPlugin);
    }

    bevy_app
        .init_resource::<BallCoords>()
        .insert_resource(ClearColor(Color::rgb_u8(111, 182, 246)))
        .add_plugins(default_plugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_systems(Startup, setup)
        .add_systems(Update, (text_update_system, ball_position).chain());

    // In this scenario, need to call the setup() of the plugins that have been registered
    // in the App manually.
    // https://github.com/bevyengine/bevy/issues/7576
    // bevy 0.11 changed: https://github.com/bevyengine/bevy/pull/8336
    #[cfg(any(target_os = "android", target_os = "ios"))]
    {
        use bevy::app::PluginsState;
        while bevy_app.plugins_state() != PluginsState::Ready {
            bevy::tasks::tick_global_task_pools_on_main_thread();
        }
        bevy_app.finish();
        bevy_app.cleanup();
        bevy_app.update();
    }

    bevy_app
}

#[cfg(any(target_os = "android", target_os = "ios"))]
pub(crate) fn change_input(app: &mut App, key_code: KeyCode, state: ButtonState) {
    let mut windows_system_state: SystemState<Query<(Entity, &mut Window)>> =
        SystemState::from_world(&mut app.world);
    let windows = windows_system_state.get_mut(&mut app.world);
    if let Ok((entity, _)) = windows.get_single() {
        let input = KeyboardInput {
            scan_code: if key_code == KeyCode::Left { 123 } else { 124 },
            state,
            key_code: Some(key_code),
            window: entity,
        };
        app.world.cell().send_event(input);
    }
}

#[cfg(any(target_os = "android", target_os = "ios"))]
#[allow(clippy::type_complexity)]
pub(crate) fn close_bevy_window(mut app: Box<App>) {
    use bevy::app::AppExit;
    let mut windows_state: SystemState<(
        Commands,
        Query<(Entity, &mut Window)>,
        EventWriter<AppExit>,
    )> = SystemState::from_world(&mut app.world);
    let (mut commands, windows, mut app_exit_events) = windows_state.get_mut(&mut app.world);
    for (window, _focus) in windows.iter() {
        commands.entity(window).despawn();
    }
    app_exit_events.send(AppExit);
    windows_state.apply(&mut app.world);

    app.update();
}
