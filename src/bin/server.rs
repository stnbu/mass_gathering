use bevy::prelude::*;
use mass_gathering::{networking::*, systems::cubic, PhysicsConfig};

fn main() {
    App::new()
        .init_resource::<InitData>()
        .insert_resource(PhysicsConfig { sims_per_frame: 5 })
        .add_startup_system(spawn_arena_view_camera)
        .add_plugin(FullGame::Server)
        .add_startup_system(cubic)
        .run();
}
