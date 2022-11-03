use bevy::prelude::*;
use bevy_rapier3d::prelude::RapierConfiguration;
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
    mut rapier_config: ResMut<RapierConfiguration>,
) {
    rapier_config.gravity = Vec3::ZERO;

    spawn_planet(
        5.0,
        Vec3::X * -5.5,
        Vec3::X * 0.2,
        Color::GOLD,
        &mut commands,
        &mut meshes,
        &mut materials,
    );
    spawn_planet(
        5.0,
        Vec3::X * 5.5,
        Vec3::X * -0.2,
        Color::SILVER,
        &mut commands,
        &mut meshes,
        &mut materials,
    );
}
