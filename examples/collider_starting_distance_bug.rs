use bevy::prelude::*;
use mass_gathering::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::MIDNIGHT_BLUE * 0.1))
        .insert_resource(SpacecraftConfig {
            stereo_enabled: false,
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
    let upper_pair = Vec3::Y * 14.0;
    let lower_pair = Vec3::Y * -14.0;

    // In regards to repo @ 935952e7067fb619ef32b6364f4bf4188b90aae8

    warn!(
        "[Collider Bug?] -- The only difference between upper and lower is the starting distance."
    );
    warn!("[Collider Bug?]    [In regards only to the _first_ planetary merge that happens]");

    spawn_planet(
        10.0,
        Vec3::X * -10.5 + upper_pair,
        Vec3::X * 3.0,
        Color::RED,
        &mut commands,
        &mut meshes,
        &mut materials,
    );
    spawn_planet(
        9.0,
        Vec3::X * 9.5 + upper_pair,
        Vec3::ZERO,
        Color::BLUE,
        &mut commands,
        &mut meshes,
        &mut materials,
    );

    spawn_planet(
        10.0,
        Vec3::X * -9.5 + lower_pair,
        Vec3::X * 3.0,
        Color::RED,
        &mut commands,
        &mut meshes,
        &mut materials,
    );
    spawn_planet(
        9.0,
        Vec3::X * 9.5 + lower_pair,
        Vec3::ZERO,
        Color::BLUE,
        &mut commands,
        &mut meshes,
        &mut materials,
    );
}
