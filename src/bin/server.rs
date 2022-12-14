use bevy::prelude::*;
use mass_gathering::{networking::*, systems::*};

fn main() {
    App::new()
        .insert_resource(testing_no_unhinhabited())
        .add_plugin(FullGame::Server)
        .run();
}
