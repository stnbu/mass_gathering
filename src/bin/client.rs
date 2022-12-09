use bevy::prelude::*;

use mass_gathering::{networking::*};

fn main() {
    App::new()
        .add_event::<ClientMessages>()
        .add_startup_system(spawn_arena_view_camera)
        .add_plugin(FullGame::Client)
        .run();
}
