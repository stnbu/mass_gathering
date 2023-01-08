use bevy::prelude::*;
use bevy_renet::renet::RenetConnectionConfig;
use bevy_renet::renet::{
    DefaultChannel, RenetServer, ServerAuthentication, ServerConfig, ServerEvent,
};
use std::{net::UdpSocket, time::SystemTime};

use crate::{networking::*, GameState, InitData};

pub fn new_renet_server() -> RenetServer {
    let server_addr = format!("{SERVER_ADDR}:{PORT_NUMBER}").parse().unwrap();
    let socket = UdpSocket::bind(server_addr).unwrap();
    let server_config =
        ServerConfig::new(64, PROTOCOL_ID, server_addr, ServerAuthentication::Unsecure);
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

pub fn setup_physics(mut commands: Commands, cli_args: Res<ServerCliArgs>) {
    let speed = cli_args.speed;
    commands.insert_resource(PhysicsConfig {
        sims_per_frame: speed,
    });
}

#[derive(Resource, Default)]
pub struct UnassignedMasses(Vec<u64>);

// FIXME: oh, so bad.
pub fn populate_unassigned_masses(
    mut unassigned_masses: ResMut<UnassignedMasses>,
    init_data: Res<InitData>,
) {
    for (mass_id, mass_init_data) in init_data.masses.iter() {
        if mass_init_data.inhabitable {
            unassigned_masses.0.push(*mass_id);
        }
    }
}

pub fn handle_server_events(
    mut server_events: EventReader<ServerEvent>,
    mut server: ResMut<RenetServer>,
    init_data: Res<InitData>,
    mut app_state: ResMut<State<GameState>>,
    mut lobby: ResMut<Lobby>,
    mut unassigned_masses: ResMut<UnassignedMasses>,
    physics_config: Res<PhysicsConfig>,
) {
    for event in server_events.iter() {
        match event {
            ServerEvent::ClientConnected(id, user_data) => {
                // FIXME: where? here? somewhere we need to "handle" clients connecting
                // to an in-progress game (which we do not allow).
                let new_id = *id;

                let client_preferences = ClientPreferences::from_user_data(user_data);
                debug!("Server got connection from new client {new_id} with preferences {client_preferences:?}");

                let inhabited_mass_id = if let Some(id) = unassigned_masses.0.pop() {
                    id
                } else {
                    warn!("Got connection from client {new_id} but we have no more uninhabited masses");
                    // FIXME: send "error" back to client?
                    return;
                };

                debug!("  sending initial data to client {new_id}");
                let message = bincode::serialize(&ServerMessage::Init(init_data.clone())).unwrap();
                server.send_message(new_id, CHANNEL, message);

                debug!("  sending physics config to {new_id}");
                let message =
                    bincode::serialize(&ServerMessage::SetPhysicsConfig(*physics_config)).unwrap();
                server.send_message(new_id, CHANNEL, message);

                debug!("  replaying existing lobby back to new client {new_id:?}");
                for (&existing_id, &client_data) in lobby.clients.iter() {
                    let message = ServerMessage::ClientJoined {
                        id: existing_id,
                        client_data,
                    };
                    server.send_message(
                        new_id,
                        DefaultChannel::Reliable,
                        bincode::serialize(&message).unwrap(),
                    );
                }

                let client_data = ClientData {
                    preferences: client_preferences,
                    inhabited_mass_id,
                };

                debug!("  now updating my lobby with ({new_id}, {client_data:?})");
                lobby.clients.insert(new_id, client_data);
                debug!("  the server now has lobby {lobby:?}");
                let message = ServerMessage::ClientJoined {
                    id: new_id,
                    client_data,
                };
                debug!("  broadcasting about new client (except to {new_id}): {message:?}");
                server.broadcast_message_except(
                    new_id,
                    DefaultChannel::Reliable,
                    bincode::serialize(&message).unwrap(),
                );
            }
            ServerEvent::ClientDisconnected(id) => {
                debug!("Server got disconnect from client {id}");
            }
        }
    }

    for client_id in server.clients_id().into_iter() {
        while let Some(message) = server.receive_message(client_id, CHANNEL) {
            let message = bincode::deserialize(&message).unwrap();
            debug!("Received message from client: {message:?}");
            match message {
                ClientMessages::Ready => {
                    let unanimous_autostart = lobby.clients.len() > 1
                        && lobby
                            .clients
                            .iter()
                            .all(|(_, data)| data.preferences.autostart);
                    if unanimous_autostart {
                        debug!("  two or more clients connected and all want to autostart.");
                    }
                    let game_full = lobby.clients.len()
                        == init_data
                            .masses
                            .iter()
                            .filter(|(_, data)| data.inhabitable)
                            .count();
                    if game_full {
                        debug!("  game has now reached max capacity.");
                    }
                    let start = unanimous_autostart || game_full;
                    let state = if start {
                        GameState::Running
                    } else {
                        GameState::Waiting
                    };
                    let set_state = ServerMessage::SetGameState(state);
                    let message = bincode::serialize(&set_state).unwrap();
                    if start {
                        debug!("Broadcasting {set_state:?}");
                        server.broadcast_message(CHANNEL, message);
                    } else {
                        // FIXME: we have inconsistency/arbitrariness in 2nd arg choice (channel)
                        debug!("Replying to client {client_id} with {set_state:?}");
                        server.send_message(client_id, DefaultChannel::Reliable, message);
                    }
                    debug!("  and setting my state to {state:?}");
                    let _ = app_state.overwrite_set(state);
                }
                ClientMessages::Rotation(rotation) => {
                    debug!("Sending rotation event for client {client_id}");
                    let client_rotation = ServerMessage::ClientRotation {
                        id: client_id,
                        rotation,
                    };
                    let message = bincode::serialize(&client_rotation).unwrap();
                    debug!("Broadcasting except to {client_id}: {client_rotation:?}");
                    server.broadcast_message_except(client_id, CHANNEL, message);
                }
            }
        }
    }
}
