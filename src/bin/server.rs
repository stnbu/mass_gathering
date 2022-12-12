use bevy::prelude::*;
use mass_gathering::{networking::*, systems::*};

fn main() {
    App::new()
        .insert_resource(cubic())
        .add_startup_system(spawn_arena_view_camera)
        .add_plugin(FullGame::Server)
        .run();
}
