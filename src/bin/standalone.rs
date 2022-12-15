use bevy::prelude::*;

use mass_gathering::networking::*;

fn main() {
    App::new().add_plugin(FullGame::Standalone).run();
}
