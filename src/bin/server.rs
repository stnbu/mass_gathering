use bevy::prelude::*;
use mass_gathering::{networking::*, systems::old_rando};

fn main() {
    App::new()
        .insert_resource(old_rando())
        .add_startup_system(spawn_arena_view_camera)
        .add_plugin(FullGame::Server)
        .run();
}
