use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(
            0xF9 as f32 / 255.0,
            0xF9 as f32 / 255.0,
            0xFF as f32 / 255.0,
        )))
        .insert_resource(Msaa::default())
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup_graphics)
        .add_startup_system(ball)
        .run();
}

fn setup_graphics(mut commands: Commands) {
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(-15.0, 8.0, 15.0)
            .looking_at(Vec3::new(-5.0, 0.0, 5.0), Vec3::Y),
        ..Default::default()
    });
}

fn ball(mut commands: Commands) {
    commands
        .spawn_bundle(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)))
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(0.5));
}
