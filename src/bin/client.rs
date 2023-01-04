use bevy::prelude::App;
use mass_gathering::networking::{FullGameClient};

fn main() {
    App::new().add_plugin(FullGameClient).run();
}
