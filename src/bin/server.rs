use bevy::prelude::*;
use bevy_renet::RenetServerPlugin;
use mass_gathering::{networking::*, systems::*, PhysicsConfig};

fn main() {
    App::new()
        .init_resource::<InitData>()
        .insert_resource(PhysicsConfig { sims_per_frame: 5 })
        .add_startup_system(spawn_arena_view_camera)
        .add_plugin(FullGame::Server)
        //
        .add_startup_system(cubic)
        .add_plugin(RenetServerPlugin::default())
        .insert_resource(new_renet_server())
        .add_system(handle_server_events)
        //
        .run();
}
