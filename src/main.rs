use bevy::prelude::App;
use mass_gathering::prelude::my_planets;
use mass_gathering::FullGame;

fn main() {
    App::new()
        .add_plugins(FullGame)
        .add_startup_system(my_planets)
        .run();
}
