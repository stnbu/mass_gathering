use bevy::prelude::*;
use bevy_renet::{
    renet::{RenetServer, ServerAuthentication, ServerConfig, ServerEvent},
    RenetServerPlugin,
};
use mass_gathering::{
    server_connection_config, setup_level, ClientChannel, PlayerCommand, PlayerInput,
    ServerChannel, ServerMessages, PORT_NUMBER, PROTOCOL_ID, SERVER_ADDR,
};
use std::{collections::HashMap, net::UdpSocket, time::SystemTime};

#[derive(Debug, Default, Resource)]
pub struct ServerLobby {
    pub players: HashMap<u64, Entity>,
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

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugin(RenetServerPlugin::default());
    app.insert_resource(ServerLobby::default());
    app.insert_resource(new_renet_server());
    app.add_system(server_update_system);
    app.add_startup_system(setup_level);
    app.run();
}

#[allow(clippy::too_many_arguments)]
fn server_update_system(
    mut server_events: EventReader<ServerEvent>,
    mut _lobby: ResMut<ServerLobby>,
    mut server: ResMut<RenetServer>,
) {
    for event in server_events.iter() {
        match event {
            ServerEvent::ClientConnected(id, _) => {
                info!("Player {} connected.", id);
                let message =
                    bincode::serialize(&ServerMessages::PlayerCreate { id: *id }).unwrap();
                server.broadcast_message(ServerChannel::ServerMessages, message);
            }
            ServerEvent::ClientDisconnected(id) => {
                info!("Player {} disconnected.", id);
                let message =
                    bincode::serialize(&ServerMessages::PlayerRemove { id: *id }).unwrap();
                server.broadcast_message(ServerChannel::ServerMessages, message);
            }
        }
    }

    for client_id in server.clients_id().into_iter() {
        while let Some(message) = server.receive_message(client_id, ClientChannel::Command) {
            let command: PlayerCommand = bincode::deserialize(&message).unwrap();
            info!("Got command from client {client_id:?}: {command:?}");
        }
        while let Some(message) = server.receive_message(client_id, ClientChannel::Input) {
            let command: PlayerInput = bincode::deserialize(&message).unwrap();
            info!("Got input from client {client_id:?}: {command:?}");
        }
    }
}
