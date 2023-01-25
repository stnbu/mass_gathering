use bevy::app::AppExit;
use bevy_renet::renet::ServerEvent;
use bevy_renet::renet::{
    DefaultChannel, RenetConnectionConfig, RenetServer, ServerAuthentication, ServerConfig,
};
use clap::Parser;
use game::*;

use std::{net::UdpSocket, time::SystemTime};

pub mod plugins;

#[derive(Parser, Resource)]
pub struct ServerCliArgs {
    #[arg(long, default_value_t = 1)]
    pub speed: u32,
    #[arg(long, default_value_t = ("").to_string())]
    pub system: String,
    #[arg(long, default_value_t = format!("{SERVER_IP}:{SERVER_PORT}"))]
    pub address: String,
    #[arg(long)]
    pub zerog: bool,
}

pub fn new_renet_server(address: String) -> RenetServer {
    let address = if let Ok(address) = format!("{address}").parse() {
        address
    } else {
        panic!("Cannot parse address `{address}`");
    };
    let socket = UdpSocket::bind(address).unwrap();
    let server_config = ServerConfig::new(64, PROTOCOL_ID, address, ServerAuthentication::Unsecure);
    RenetServer::new(
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap(),
        server_config,
        RenetConnectionConfig::default(),
        socket,
    )
    .unwrap()
}

pub fn handle_server_events(
    mut server_events: EventReader<ServerEvent>,
    mut server: ResMut<RenetServer>,
    mut app_state: ResMut<State<resources::GameState>>,
    mut game_config: ResMut<resources::GameConfig>,
    mut exit: EventWriter<AppExit>,
) {
    for event in server_events.iter() {
        match event {
            &ServerEvent::ClientConnected(client_id, _) => {
                match game_config.get_assigned_mass_id(client_id) {
                    Ok(mass_id) => {
                        debug!("Client {client_id} assigned mass {mass_id}");
                        if game_config.is_capacity() {
                            debug!("Game is at capacity, sending configuration to clients");
                            server.broadcast_message(
                                DefaultChannel::Reliable,
                                bincode::serialize(&events::ToClient::SetGameConfig(
                                    game_config.clone(),
                                ))
                                .unwrap(),
                            );
                            let state = resources::GameState::Running;
                            server.broadcast_message(
                                DefaultChannel::Reliable,
                                bincode::serialize(&events::ToClient::SetGameState(state)).unwrap(),
                            );
                            let _ = app_state.overwrite_set(state);
                        } else {
                            let state = resources::GameState::Waiting;
                            server.broadcast_message(
                                DefaultChannel::Reliable,
                                bincode::serialize(&events::ToClient::SetGameState(state)).unwrap(),
                            );
                        }
                    }
                    Err(err) => panic!("{err}"),
                }
            }
            &ServerEvent::ClientDisconnected(_) => {
                exit.send(AppExit);
            }
        }
    }

    for client_id in server.clients_id().into_iter() {
        while let Some(message) = server.receive_message(client_id, DefaultChannel::Reliable) {
            let message = bincode::deserialize(&message).unwrap();
            match message {
                events::ToServer::Rotation(rotation) => {
                    let inhabitant_rotation = events::ToClient::InhabitantRotation {
                        client_id,
                        rotation,
                    };
                    let message = bincode::serialize(&inhabitant_rotation).unwrap();
                    server.broadcast_message_except(client_id, DefaultChannel::Reliable, message);
                }
                events::ToServer::ProjectileFired(projectile_flight) => {
                    let projectile_fired = events::ToClient::ProjectileFired(projectile_flight);
                    let message = bincode::serialize(&projectile_fired).unwrap();
                    server.broadcast_message(DefaultChannel::Reliable, message);
                }
            }
        }
    }
}
