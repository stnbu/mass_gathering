use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

fn main() {
    let mut app = App::new();

    app.insert_resource(ClearColor(Color::rgb(
        0xF9 as f32 / 255.0,
        0xF9 as f32 / 255.0,
        0xFF as f32 / 255.0,
    )))
    .insert_resource(Msaa::default())
    .add_plugins(DefaultPlugins)
    .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
    .add_plugin(RapierDebugRenderPlugin::default())
    .add_startup_system(max_win)
    .add_startup_system(setup_graphics)
    .add_startup_system(setup_physics)
    .run();
}

fn max_win(mut windows: ResMut<Windows>) {
    for window in windows.iter_mut() {
        window.set_maximized(true);
    }
}

fn setup_graphics(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle {
        transform: Transform::from_xyz(0.0, 200.0, 0.0),
        ..default()
    });
}

pub fn setup_physics(mut commands: Commands) {
    // Floor and Ceiling
    for y_position in [630.0, -290.0] {
        commands
            .spawn_bundle(TransformBundle::from(Transform::from_xyz(
                0.0, y_position, 0.0,
            )))
            .insert(Collider::cuboid(450.0, 10.0))
            .insert(Restitution::coefficient(10.0));
    }

    // Walls
    for x_position in [-460.0, 460.0] {
        commands
            .spawn_bundle(TransformBundle::from(Transform::from_xyz(
                x_position, 170.0, 0.0,
            )))
            .insert(Collider::cuboid(10.0, 470.0))
            .insert(Restitution::coefficient(10.0));
    }

    // Paddles
    for y in [500.0, 400.0, 300.0, 200.0, 100.0, 0.0, -100.0, -200.0] {
        for x in [
            -400.0, -300.0, -200.0, -100.0, 0.0, 100.0, 200.0, 300.0, 400.0,
        ] {
            commands
                .spawn_bundle(TransformBundle::from(Transform::from_xyz(x, y, 0.0)))
                .insert(RigidBody::Dynamic)
                .insert(LockedAxes::TRANSLATION_LOCKED)
                .insert(Collider::cuboid(48.0, 5.0))
                .insert(Friction::new(0.05));
        }
    }

    // Bwoalls
    let mut rng = rand::thread_rng();
    for x in [
        -420.0, -320.0, -220.0, -120.0, -20.0, 80.0, 180.0, 280.0, 380.0,
    ] {
        let wiggle: u8 = rng.gen::<u8>() % 20;
        commands
            .spawn_bundle(TransformBundle::from(
                Transform::from_xyz(x + wiggle as f32, 600.0, 0.0)
                    .with_rotation(Quat::from_rotation_z(1.0)),
            ))
            .insert(RigidBody::Dynamic)
            .insert(Collider::cuboid(3.0, 3.0))
            .insert(Ccd::enabled())
            .insert(ColliderMassProperties::Density(0.1 * wiggle as f32))
            .insert(Friction::new(1.0 / wiggle as f32));
    }
}
