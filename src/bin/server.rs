use bevy::prelude::*;
use mass_gathering::{networking::*, systems::*};

fn main() {
    App::new()
        .insert_resource(testing_no_unhinhabited())
        .add_startup_system(spawn_arena_view_camera)
        .add_plugin(FullGame::Server)
        .run();
}
