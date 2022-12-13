use bevy::prelude::*;

use mass_gathering::networking::*;

fn main() {
    App::new()
        .add_event::<ClientMessages>()
        .add_plugin(FullGame::Client)
        .run();
}
