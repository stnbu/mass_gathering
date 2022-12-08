use bevy::prelude::*;
use bevy_renet::{
    renet::{RenetServer, ServerAuthentication, ServerConfig, ServerEvent},
    RenetServerPlugin,
};
use mass_gathering::{
    networking::*, server_connection_config, spawn_arena_view_camera, systems::*, ClientChannel,
    FullGame, GameState, InitData, PhysicsConfig, ServerChannel, ServerMessages, PORT_NUMBER,
    PROTOCOL_ID, SERVER_ADDR,
};

use std::net::UdpSocket;
use std::time::SystemTime;

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
