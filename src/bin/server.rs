use bevy::prelude::*;
use mass_gathering::{systems::cubic, FullGame, PhysicsConfig};

fn add_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(10.0, 10.0, 10.0).looking_at(-Vec3::Z, Vec3::Y),
        ..Default::default()
    });
}

fn main() {
    App::new()
        .insert_resource(PhysicsConfig { sims_per_frame: 4 })
        .add_plugins(FullGame)
        .add_startup_system(add_camera)
        .add_startup_system(cubic)
        .run();
}
