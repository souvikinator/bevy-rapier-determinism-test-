use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
    sprite::MaterialMesh2dBundle,
    window::{PresentMode, PrimaryWindow},
};
use bevy_rapier2d::prelude::*;

#[derive(Resource, Default)]
pub struct BallCoords(Vec2);

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct BallPositionText;

#[derive(Component)]
pub struct Ball;

pub fn setup(
    mut commands: Commands,
    mut rapier_config: ResMut<RapierConfiguration>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    rapier_config.timestep_mode = TimestepMode::Fixed {
        dt: 1. / 60.,
        substeps: 1,
    };

    commands
        .spawn(Collider::cuboid(500.0, 100.0))
        .insert(MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::new(1000., 200.)).into(),
            material: materials.add(ColorMaterial::from(Color::rgb(0.5, 0.5, 0.5))),
            transform: Transform::from_xyz(0.0, -250.0, 0.0),
            ..default()
        });

    commands
        .spawn((RigidBody::Dynamic, Ball))
        .insert(Collider::cuboid(10.0, 10.0))
        .insert(Restitution::coefficient(0.7))
        .insert(MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::new(20., 20.)).into(),
            material: materials.add(ColorMaterial::from(Color::RED)),
            transform: Transform::from_xyz(0.0, 400.0, 0.0),
            ..default()
        });

    // Text with multiple sections
    commands.spawn((
        // Create a TextBundle that has a Text with a list of sections.
        TextBundle::from_sections([
            TextSection::new(
                "POS:\n",
                TextStyle {
                    // This font is loaded and will be used instead of the default font.
                    font: asset_server.load("fonts/RobotoMono-Regular.ttf"),
                    font_size: 30.0,
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load("fonts/RobotoMono-Regular.ttf"),
                font_size: 30.0,
                color: Color::GOLD,
            }), // Set the style of the TextBundle itself.
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            left: Val::Px(15.0),
            ..Default::default()
        }),
        BallPositionText,
    ));

    commands.spawn((Camera2dBundle::default(), MainCamera));
}

pub fn text_update_system(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<BallPositionText>>,
) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                text.sections[1].value = format!("{value:.2}");
            }
        }
    }
}

pub fn ball_position(
    mut mycoords: ResMut<BallCoords>,
    q_ball: Query<&Transform, With<Ball>>,
    mut q_label: Query<&mut Text, With<BallPositionText>>,
) {
    if let Some(ball_transform) = q_ball.iter().next() {
        for mut text in &mut q_label {
            mycoords.0 = ball_transform.translation.truncate();
            text.sections[1].value = format!("({},{})", mycoords.0.x, mycoords.0.y);
            println!("{:?}", mycoords.0);
        }
    }
}

// fn my_cursor_system(
//     mut mycoords: ResMut<MyWorldCoords>,
//     // query to get the window (so we can read the current cursor position)
//     q_window: Query<&Window, With<PrimaryWindow>>,
//     // query to get camera transform
//     q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
// ) {
//     // get the camera info and transform
//     // assuming there is exactly one main camera entity, so Query::single() is OK
//     let (camera, camera_transform) = q_camera.single();

//     // There is only one primary window, so we can similarly get it from the query:
//     let window = q_window.single();

//     // check if the cursor is inside the window and get its position
//     // then, ask bevy to convert into world coordinates, and truncate to discard Z
//     if let Some(world_position) = window
//         .cursor_position()
//         .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
//         .map(|ray| ray.origin.truncate())
//     {
//         mycoords.0 = world_position;
//         eprintln!("World coords: {}/{}", world_position.x, world_position.y);
//     }
// }
