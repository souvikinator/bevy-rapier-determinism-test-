use bevy::{diagnostic::DiagnosticsStore, prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb_u8(111, 182, 246)))
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_systems(Startup, setup)
        .add_systems(Update, text_update_system)
        .run();
}

#[derive(Component)]
struct BallPositionText;

#[derive(Component)]
struct Ball;

fn setup(
    mut commands: Commands,
    mut rapier_config: ResMut<RapierConfiguration>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // rapier_config.timestep_mode = TimestepMode::Fixed {
    //     dt: 1. / 60.,
    //     substeps: 1,
    // };

    commands
        .spawn(Collider::cuboid(500.0, 100.0))
        .insert(MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::new(1000., 200.)).into(),
            material: materials.add(ColorMaterial::from(Color::rgb(0.5, 0.5, 0.5))),
            transform: Transform::from_xyz(0.0, -250.0, 0.0),
            ..default()
        });

    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::cuboid(10.0, 10.0))
        .insert(Restitution::coefficient(0.7))
        .insert(MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::new(20., 20.)).into(),
            material: materials.add(ColorMaterial::from(Color::RED)),
            transform: Transform::from_xyz(0.0, 400.0, 0.0),
            ..default()
        });

    commands.spawn((
        // Create a TextBundle that has a Text with a single section.
        TextBundle::from_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            "hello\nbevy!",
            TextStyle {
                // This font is loaded and will be used instead of the default font.
                font: asset_server.load("fonts/RobotoMono-Regular.ttf"),
                font_size: 100.0,
                ..default()
            },
        ) // Set the justification of the Text
        .with_text_justify(JustifyText::Center)
        // Set the style of the TextBundle itself.
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            right: Val::Px(5.0),
            ..default()
        }),
        BallPositionText,
    ));

    commands.spawn(Camera2dBundle::default());
}

fn text_update_system(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<BallPositionText>>,
) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                // Update the value of the second section
                text.sections[1].value = format!("{value:.2}");
            }
        }
    }
}
