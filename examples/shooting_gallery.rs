use bevy::log::LogSettings;

use bevy::prelude::*;
use bevy_rapier3d::prelude::{NoUserData, RapierConfiguration, RapierPhysicsPlugin};
use mass_gathering::prelude::*;

fn main() {
    let mut app = App::new();
    app.insert_resource(ClearColor(Color::WHITE * 0.8))
        .insert_resource(SpaceCraftConfig {
            show_debug_markers: true,
            show_impact_explosions: false,
            projectile_radius: 0.02,
        });

    app.insert_resource(LogSettings {
        filter: "debug".into(),
        level: bevy::log::Level::DEBUG,
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
    .add_system(collision_events)
    .add_system(timer_despawn)
    .add_system(do_blink)
    .add_plugin(RapierPhysicsPlugin::<NoUserData>::default());

    app.run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut rapier_config: ResMut<RapierConfiguration>,
) {
    rapier_config.gravity = Vec3::ZERO;

    spawn_planet(
        5.001,
        Vec3::X * -15.0,
        Vec3::X * 0.8,
        Color::SILVER,
        &mut commands,
        &mut meshes,
        &mut materials,
    );

    spawn_planet(
        5.0,
        Vec3::X * 15.0,
        Vec3::X * -0.8,
        Color::GOLD,
        &mut commands,
        &mut meshes,
        &mut materials,
    );
}
