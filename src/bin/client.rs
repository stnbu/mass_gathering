use bevy::prelude::App;
use mass_gathering::networking::{ClientMessages, FullGameClient};

fn main() {
    App::new()
        .add_event::<ClientMessages>()
        .add_plugin(FullGameClient)
        .run();
}
