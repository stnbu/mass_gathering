use bevy::prelude::{App, ClearColor, Color, Transform, Vec3};
use mass_gathering::prelude::{PhysicsConfig, SpacecraftConfig};
use mass_gathering::FullGame;

fn my_planets() {}

fn main() {
    let d = 60.0 / 3.0_f32.powf(0.5); // about right for my_planets
    App::new()
        .insert_resource(ClearColor(Color::MIDNIGHT_BLUE * 0.1))
        .insert_resource(PhysicsConfig {
            sims_per_frame: 1,
            trails: true,
            trail_ttl: 2500 * 5,
        })
        .insert_resource(SpacecraftConfig {
            start_transform: Transform::from_xyz(d, d, d).looking_at(Vec3::ZERO, Vec3::Y),
            impact_magnitude: 5.0,
            ..Default::default()
        })
        .add_plugins(FullGame)
        .add_startup_system(my_planets)
        .run();
}
