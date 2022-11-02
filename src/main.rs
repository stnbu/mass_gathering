use bevy::prelude::App;
use mass_gathering::prelude::{my_planets, SpacecraftConfig};
use mass_gathering::FullGame;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let stereo_enabled = if let Some(arg) = args.get(1) {
        matches!(arg.as_str(), "--stereo")
    } else {
        false
    };

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
