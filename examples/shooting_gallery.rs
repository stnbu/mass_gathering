use bevy::log::LogSettings;
use bevy::prelude::*;
use mass_gathering::prelude::*;

/*
Reference:

Masses:

 earth - 5.97217×10^24 kg
 moon  - 7.34200×10^22 kg
 sun   - 1.98850×10^30 kg

*/
fn main() {
    App::new()
        .insert_resource(LogSettings {
            filter: "warn,mass_gathering=debug".into(),
            level: bevy::log::Level::DEBUG,
        })
        .insert_resource(ClearColor(Color::MIDNIGHT_BLUE * 0.1))
        .insert_resource(SpacecraftConfig {
            stereo_enabled: false,
            stereo_iod: 2.0,
            start_transform: Transform::from_xyz(0.0, 0.0, 50.0).looking_at(Vec3::ZERO, Vec3::Y),
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
    let sun_radius = 8.0;
    let earth_color = Color::rgb(23.0, 57.0, 61.0);
    //let sun_color = Color::rgb(244.0, 233.0, 155.0);
    let moon_color = Color::rgb(149.0, 136.0, 132.0);

    // The sun, beautiful
    spawn_planet(
        sun_radius,
        Vec3::ZERO,
        Vec3::ZERO,
        earth_color,
        &mut commands,
        &mut meshes,
        &mut materials,
    );
    // The moon, even more beautiful
    spawn_planet(
        1.0,
        Vec3::X * sun_radius * 3.0,
        Vec3::Z * 1.588,
        moon_color,
        &mut commands,
        &mut meshes,
        &mut materials,
    );
}
