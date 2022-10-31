use bevy::prelude::*;
use bevy_rapier3d::prelude::{NoUserData, RapierConfiguration, RapierPhysicsPlugin};
use mass_gathering::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::WHITE * 0.8))
        .insert_resource(SpacecraftConfig::default())
        .add_plugins(DefaultPlugins)
        .add_system(freefall)
        .add_system(collision_events)
        .add_system(handle_projectile_engagement)
        .add_system(handle_projectile_flight)
        .add_system(animate_projectile_explosion)
        .add_startup_system(setup)
        .add_startup_system(spacecraft_setup)
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
        8.0,
        Vec3::X * -15.0,
        Vec3::ZERO,
        Color::RED,
        &mut commands,
        &mut meshes,
        &mut materials,
    );

    spawn_planet(
        5.0,
        Vec3::X * 6.0,
        Vec3::X * -1.0,
        Color::BLUE,
        &mut commands,
        &mut meshes,
        &mut materials,
    );

    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(0.0, 30.0, 40.0),
        point_light: PointLight {
            intensity: 250000.0,
            range: 1000.0,
            ..Default::default()
        },
        ..Default::default()
    });
}
