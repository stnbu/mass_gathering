use bevy::prelude::App;
use mass_gathering::prelude::{my_planets, SpacecraftConfig};
use mass_gathering::FullGame;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::MIDNIGHT_BLUE * 0.1))
        .insert_resource(SpacecraftConfig {
            stereo_enabled: false,
            stereo_iod: 2.0,
            ..Default::default()
        })
        .add_plugins(FullGame)
        .add_startup_system(my_planets)
        .run();
}
