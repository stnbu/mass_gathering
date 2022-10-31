use bevy::prelude::App;
use mass_gathering::prelude::{my_planets, SpacecraftConfig};
use mass_gathering::FullGame;

fn main() {
    App::new()
        .insert_resource(SpacecraftConfig {
            stereo_enabled: false,
            stereo_iod: 6.0,
            ..Default::default()
        })
        .add_plugins(FullGame)
        .add_startup_system(my_planets)
        .run();
}
