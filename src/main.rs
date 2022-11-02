use bevy::prelude::App;
use mass_gathering::prelude::{my_planets, SpacecraftConfig};
use mass_gathering::FullGame;

use std::env;

fn main() {
    let stereo_enabled = env::var("STEREO").is_ok();

    App::new()
        .insert_resource(SpacecraftConfig {
            stereo_enabled,
            stereo_iod: 2.0,
            ..Default::default()
        })
        .add_plugins(FullGame)
        .add_startup_system(my_planets)
        .run();
}
