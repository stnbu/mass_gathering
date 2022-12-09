use bevy::prelude::*;
use mass_gathering::{networking::*, systems::cubic};

fn main() {
    App::new()
        .init_resource::<InitData>()
        .add_startup_system(spawn_arena_view_camera)
        .add_plugin(FullGame::Server)
        .add_startup_system(cubic)
        .run();
}
