use bevy::log::LogSettings;
use bevy::prelude::*;
use mass_gathering::prelude::*;

fn main() {
    App::new()
        .insert_resource(LogSettings {
            filter: "warn,mass_gathering=debug".into(),
            level: bevy::log::Level::DEBUG,
        })
        .insert_resource(ClearColor(Color::MIDNIGHT_BLUE * 0.1))
        .insert_resource(SpacecraftConfig {
            stereo_enabled: false,
            start_transform: Transform::from_xyz(0.0, 0.0, 100.0).looking_at(Vec3::ZERO, Vec3::Y),
            impact_magnitude: 5.0,
            ..Default::default()
        })
        .insert_resource(PhysicsConfig {
            sims_per_frame: 1,
            trails: true,
            trail_ttl: 10_000,
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
    /*
    You are at Z = 100 blue is between you and red. Blue is heading toward red (which it fully obscures).

    Shoot at blue with great vigor (fill it with bullets) as it heads towards its collision with red.

    When blue collides with red and gets despawned, all of the shooting-related stuff that is associated
    with blue need to be "dealt with". Inflight projectiles, explosion animations, systems that may run
    after (by mistake), etc.

    As of 53abb5f5ae0092812b7548ed46d5ed349df05d27
      * A crash has been observed!
      * In-flight projectiles halt in flight and instead (!) log a warning.
    */

    spawn_planet(
        10.0,
        Vec3::ZERO,
        Vec3::ZERO,
        Color::RED,
        &mut commands,
        &mut meshes,
        &mut materials,
    );
    spawn_planet(
        9.0,
        Vec3::Z * 30.0,
        Vec3::Z * -2.0,
        Color::BLUE,
        &mut commands,
        &mut meshes,
        &mut materials,
    );
}
