use bevy::log::LogSettings;
use bevy::prelude::*;
use mass_gathering::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // .insert_resource(LogSettings {
        //     filter: "warn,mass_gathering=debug".into(),
        //     level: bevy::log::Level::DEBUG,
        // })
        .insert_resource(ClearColor(Color::MIDNIGHT_BLUE * 0.1))
        .insert_resource(SpacecraftConfig {
            start_transform: Transform::from_xyz(0.0, 0.0, 50.0).looking_at(Vec3::ZERO, Vec3::Y),
            impact_magnitude: 100.0,
            projectile_radius: 0.05,
            ..Default::default()
        })
        .insert_resource(PhysicsConfig {
            sims_per_frame: 10,
            trails: true,
            trail_ttl: 20_000,
        })
        .add_plugins(FullGame)
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let redish = *Color::RED.set_a(0.9);
    let blueish = *Color::BLUE.set_a(0.9);
    spawn_planet(
        10.0,
        Vec3::X * 10.01,
        Vec3::ZERO,
        redish,
        &mut commands,
        &mut meshes,
        &mut materials,
    );

    spawn_planet(
        10.0,
        Vec3::X * -10.01,
        Vec3::ZERO,
        blueish,
        &mut commands,
        &mut meshes,
        &mut materials,
    );
}
