use bevy::prelude::*;
use mass_gathering::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::MIDNIGHT_BLUE * 0.1))
        .insert_resource(SpacecraftConfig {
            start_transform: Transform::from_xyz(0.0, 0.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
            impact_magnitude: 100.0,
            projectile_radius: 0.05,
            start_speed: -0.5 * 10.0 * 0.9,
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
    let one = Color::BEIGE;
    //let the_other = Color::WHITE;
    spawn_planet(
        2.0,
        Vec3::ZERO,
        Vec3::Z * 0.5,
        one,
        &mut commands,
        &mut meshes,
        &mut materials,
    );
}
