use bevy::prelude::App;
use mass_gathering::Game;

fn main() {
    App::new().add_plugin(Game).run();
}
