use bevy::prelude::App;
use mass_gathering::FullGame;

fn main() {
    App::new().add_plugins(FullGame).run();
}
