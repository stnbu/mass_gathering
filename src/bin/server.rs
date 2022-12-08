use bevy::prelude::*;
use bevy_renet::{
    renet::{RenetServer, ServerAuthentication, ServerConfig, ServerEvent},
    RenetServerPlugin,
};
use mass_gathering::{
    server_connection_config, spawn_arena_view_camera, systems::*, ClientChannel, FullGame,
    GameState, InitData, PhysicsConfig, ServerChannel, ServerMessages, PORT_NUMBER, PROTOCOL_ID,
    SERVER_ADDR,
};
use std::net::UdpSocket;
use std::time::SystemTime;

fn main() {
    App::new()
        .init_resource::<InitData>()
        .insert_resource(PhysicsConfig { sims_per_frame: 5 })
        .add_plugin(FullGame::Server)
        .add_startup_system(spawn_arena_view_camera)
        .add_startup_system(cubic)
        .add_plugin(RenetServerPlugin::default())
        .insert_resource(new_renet_server())
        .add_system(handle_server_events)
        .run();
}

fn new_renet_server() -> RenetServer {
    let server_addr = format!("{SERVER_ADDR}:{PORT_NUMBER}").parse().unwrap();
    let socket = UdpSocket::bind(server_addr).unwrap();
    let connection_config = server_connection_config();
    let server_config =
        ServerConfig::new(64, PROTOCOL_ID, server_addr, ServerAuthentication::Unsecure);
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    RenetServer::new(current_time, server_config, connection_config, socket).unwrap()
}

fn handle_server_events(
    mut server_events: EventReader<ServerEvent>,
    mut server: ResMut<RenetServer>,
    init_data: Res<InitData>,
    mut app_state: ResMut<State<GameState>>,
) {
    for event in server_events.iter() {
        match event {
            ServerEvent::ClientConnected(id, _) => {
                info!("client {id} connected");
                let message = bincode::serialize(&ServerMessages::Init(init_data.clone())).unwrap();
                info!("sending initial data to client {id}");
                server.send_message(*id, ServerChannel::ServerMessages, message);
            }
            ServerEvent::ClientDisconnected(id) => {
                info!("client {id} disconnected");
            }
        }
    }

    for client_id in server.clients_id().into_iter() {
        while let Some(message) = server.receive_message(client_id, ClientChannel::ClientMessages) {
            match message {
                _ => {
                    // FIXME: we need to do this "when everybody is ready"
                    let state = GameState::Running;
                    let set_state = ServerMessages::SetGameState(state);
                    let message = bincode::serialize(&set_state).unwrap();
                    info!("Broadcasting {set_state:?}");
                    server.broadcast_message(ServerChannel::ServerMessages, message);
                    info!("  and setting my state to {state:?}");
                    let _ = app_state.overwrite_set(state);
                }
            }
        }
    }
}
