// FIXME: Consider disconnecting clients and more upon exit a la
// https://bevy-cheatbook.github.io/programming/states.html

use bevy::app::AppExit;
use game::*;
// We rename this because it sounds too much like one of _our_ events (confusing).
use bevy_renet::renet::ServerEvent;
use bevy_renet::renet::{
    DefaultChannel, RenetConnectionConfig, RenetServer, ServerAuthentication, ServerConfig,
};
use std::{net::UdpSocket, time::SystemTime};

pub mod plugins;

// Only used by server, kinda hacky
#[derive(Resource, Default)]
pub struct WhatToCallThis {
    unassigned_mass_ids: Vec<u64>,
    unconfirmed: Vec<u64>,
}

pub fn populate_unassigned_masses(
    mut unassigned_masses: ResMut<WhatToCallThis>,
    init_data: Res<resources::InitData>,
) {
    for (mass_id, mass_init_data) in init_data.masses.iter() {
        if mass_init_data.inhabitable {
            unassigned_masses.0.push(*mass_id);
        }
    }
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

pub fn setup_game(mut commands: Commands, cli_args: Res<resources::ServerCliArgs>) {
    let speed = cli_args.speed;
    let zerog = cli_args.zerog;
    commands.insert_resource(resources::GameConfig {
        physics_config: resources::PhysicsConfig { speed, zerog },
        ..Default::default()
    });
}

pub fn handle_server_events(
    mut server_events: EventReader<ServerEvent>,
    mut server: ResMut<RenetServer>,
    mut app_state: ResMut<State<resources::GameState>>,
    mut game_config: ResMut<resources::GameConfig>,
    mut unassigned_masses: ResMut<WhatToCallThis>,
    mut exit: EventWriter<AppExit>,
) {
    for event in server_events.iter() {
        match event {
            &ServerEvent::ClientConnected(id, _) => {
                debug!("Client {id} has connected!");
                if let Some(mass_id) = unassigned_masses.0.pop() {
                    game_config.client_mass_map.insert(id, mass_id);
                    server.broadcast_message(
                        DefaultChannel::Reliable,
                        bincode::serialize(&events::ToClient::SetGameConfig(game_config.clone()))
                            .unwrap(),
                    );
                    debug!("Broadcasting current game config. Anticipating a `Ready` response from all.");
                } else {
                    debug!("Client {id} connected but no more assignable masses");
                };
            }
            &ServerEvent::ClientDisconnected(id) => {
                debug!(
                    "Server got disconnect from client {id} ({}). Quiting Bevy app.",
                    to_nick(id).trim_end()
                );
                exit.send(AppExit);
            }
        }
    }

    for client_id in server.clients_id().into_iter() {
        while let Some(message) = server.receive_message(client_id, DefaultChannel::Reliable) {
            let message = bincode::deserialize(&message).unwrap();
            trace!("Received message from client {client_id}: {message:?}");
            match message {
                events::ToServer::Ready => {
                    let inhabited_mass_id = if let Some(mass_id) = unassigned_masses.0.pop() {
                        mass_id
                    } else {
                        debug!("No more assignable masses");
                        return;
                    };
                    let client_data = resources::ClientData { inhabited_mass_id };
                    game_config.clients.insert(client_id, client_data);

                    server.broadcast_message(
                        DefaultChannel::Reliable,
                        bincode::serialize(&events::ToClient::ClientJoined {
                            id: client_id,
                            client_data,
                        })
                        .unwrap(),
                    );

                    let start = game_config.clients.len()
                        == init_data
                            .masses
                            .iter()
                            .filter(|(_, data)| data.inhabitable)
                            .count();
                    if start {
                        assert!(unassigned_masses.0.is_empty());
                        let state = resources::GameState::Running;
                        let set_state = events::ToClient::SetGameState(state);
                        let message = bincode::serialize(&set_state).unwrap();
                        server.broadcast_message(DefaultChannel::Reliable, message);
                        let _ = app_state.overwrite_set(state);
                    } else {
                        assert!(!unassigned_masses.0.is_empty());
                        let state = resources::GameState::Waiting;
                        let set_state = events::ToClient::SetGameState(state);
                        let message = bincode::serialize(&set_state).unwrap();
                        server.send_message(client_id, DefaultChannel::Reliable, message);
                        let _ = app_state.overwrite_set(state);
                    }
                }
                events::ToServer::Rotation(rotation) => {
                    trace!("Sending rotation event for client {client_id}");
                    let inhabitant_rotation = events::ToClient::InhabitantRotation {
                        client_id,
                        rotation,
                    };
                    let message = bincode::serialize(&inhabitant_rotation).unwrap();
                    trace!("Broadcasting except to {client_id}: {inhabitant_rotation:?}");
                    server.broadcast_message_except(client_id, DefaultChannel::Reliable, message);
                }
                events::ToServer::ProjectileFired(projectile_flight) => {
                    let projectile_fired = events::ToClient::ProjectileFired(projectile_flight);
                    let message = bincode::serialize(&projectile_fired).unwrap();
                    debug!("Broadcasting {projectile_fired:?}");
                    server.broadcast_message(DefaultChannel::Reliable, message);
                }
            }
        }
    }
}
