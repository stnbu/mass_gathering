use bevy::prelude::*;
use mass_gathering::prelude::*;
fn main() {
    App::new()
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

    // The sun, beautiful
    spawn_planet(
        sun_radius,
        Vec3::ZERO,
        Vec3::ZERO,
        Color::GOLD,
        &mut commands,
        &mut meshes,
        &mut materials,
    );
    // The moon, even more beautiful
    spawn_planet(
        1.0,
        Vec3::X * sun_radius * 3.0,
        Vec3::Z * 20.0,
        Color::SILVER,
        &mut commands,
        &mut meshes,
        &mut materials,
    );
}
