use bevy::prelude::*;
use bevy_rapier3d::prelude::{NoUserData, RapierConfiguration, RapierPhysicsPlugin};
use mass_gathering::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::WHITE * 0.8))
        .insert_resource(SpaceCraftConfig {
            show_debug_markers: true,
        })
        .add_plugins(DefaultPlugins)
        .add_system(move_forward)
        .add_system(steer)
        .add_system(handle_projectile_engagement)
        .add_system(handle_projectile_flight)
        .add_system(animate_projectile_explosion)
        .add_startup_system(setup)
        .add_startup_system(spacecraft_setup)
        .add_system(freefall)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut rapier_config: ResMut<RapierConfiguration>,
) {
    rapier_config.gravity = Vec3::ZERO;

    spawn_planet(
        5.0,
        Vec3::X * -6.0,
        Vec3::Z * -1.0,
        Color::SILVER,
        &mut commands,
        &mut meshes,
        &mut materials,
    );

    spawn_planet(
        5.0,
        Vec3::X * 6.0,
        Vec3::Z * 1.0,
        Color::GOLD,
        &mut commands,
        &mut meshes,
        &mut materials,
    );
}
