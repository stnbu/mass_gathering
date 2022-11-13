use bevy::prelude::{App, ClearColor, Color, DefaultPlugins, Transform, Vec3};
use mass_gathering::prelude::{my_planets, PhysicsConfig, SpacecraftConfig};
use mass_gathering::FullGame;

fn main() {
    let d = 60.0 / 3.0_f32.powf(0.5); // about right for my_planets
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::MIDNIGHT_BLUE * 0.1))
        .insert_resource(PhysicsConfig {
            sims_per_frame: 1,
            trails: true,
            trail_ttl: 2500 * 5,
        })
        .insert_resource(SpacecraftConfig {
            stereo_enabled: false,
            start_transform: Transform::from_xyz(d, d, d).looking_at(Vec3::ZERO, Vec3::Y),
            impact_magnitude: 100.0,
            ..Default::default()
        })
        .add_plugins(FullGame)
        .add_startup_system(my_planets)
        .run();
}
