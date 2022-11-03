use bevy::prelude::{App, ClearColor, Color};
use mass_gathering::prelude::{my_planets, PhysicsConfig, SpacecraftConfig};
use mass_gathering::FullGame;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::MIDNIGHT_BLUE * 0.1))
        .insert_resource(PhysicsConfig {
            sims_per_frame: 150,
            ..Default::default()
        })
        .insert_resource(SpacecraftConfig {
            stereo_enabled: false,
            stereo_iod: 2.0,
            recoil: 0.025,
            ..Default::default()
        })
        .add_plugins(FullGame)
        .add_startup_system(my_planets)
        .run();
}
