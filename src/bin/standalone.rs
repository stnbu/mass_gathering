use bevy::prelude::*;
use mass_gathering::FullGameStandalone;

fn main() {
    App::new().add_plugin(FullGameStandalone).run();
}
