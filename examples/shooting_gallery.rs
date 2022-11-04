use bevy::prelude::*;
use mass_gathering::prelude::*;
fn main() {
    App::new()
        .insert_resource(SpacecraftConfig {
            start_transform: Transform::from_xyz(0.0, 0.0, 24.0).looking_at(Vec3::ZERO, Vec3::Y),
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
    let sun_color = Color::rgb(244.0, 233.0, 155.0);
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
