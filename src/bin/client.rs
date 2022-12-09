use bevy::prelude::*;

use mass_gathering::{networking::*, PhysicsConfig};

fn main() {
    App::new()
        .add_event::<ClientMessages>()
        .insert_resource(PhysicsConfig { sims_per_frame: 5 })
        .add_startup_system(spawn_arena_view_camera)
        .add_plugin(FullGame::Client)
        .run();
}
