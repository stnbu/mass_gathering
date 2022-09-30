use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(
            0xF9 as f32 / 255.0,
            0xF9 as f32 / 255.0,
            0xFF as f32 / 255.0,
        )))
        .insert_resource(Msaa::default())
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup_graphics)
        .add_startup_system(setup_physics)
        .run();
}

fn setup_graphics(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle {
        transform: Transform::from_xyz(0.0, 200.0, 0.0),
        ..default()
    });
}

pub fn setup_physics(mut commands: Commands) {
    for y in [500.0, 300.0, 100.0] {
        for x in [-400.0, -200.0, 0.0, 200.0, 400.0] {
            commands
                .spawn_bundle(TransformBundle::from(Transform::from_xyz(x, y, 0.0)))
                .insert(RigidBody::Dynamic)
                .insert(LockedAxes::TRANSLATION_LOCKED)
                .insert(Collider::cuboid(95.0, 10.0));
        }
    }

    /*
     * A tilted cuboid that cannot rotate.
     */
    commands
        .spawn_bundle(TransformBundle::from(
            Transform::from_xyz(50.0, 800.0, 0.0).with_rotation(Quat::from_rotation_z(1.0)),
        ))
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(3.0, 3.0))
        .insert(ColliderMassProperties::Density(5.0));
}
