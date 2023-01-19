use crate::*;
// We rename this because it sounds too much like one of _our_ events (confusing).
use bevy_renet::renet::ServerEvent as RenetServerEvent;
use bevy_renet::renet::{
    DefaultChannel, RenetConnectionConfig, RenetServer, ServerAuthentication, ServerConfig,
};
use std::{net::UdpSocket, time::SystemTime};

// Only used by server, kinda hacky
#[derive(Resource, Default)]
pub struct UnassignedMasses(Vec<u64>);

pub fn populate_unassigned_masses(
    mut unassigned_masses: ResMut<UnassignedMasses>,
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

pub fn setup_physics(mut commands: Commands, cli_args: Res<resources::ServerCliArgs>) {
    let speed = cli_args.speed;
    let zerog = cli_args.zerog;
    commands.insert_resource(physics::PhysicsConfig {
        sims_per_frame: speed,
        zerog,
    });
}

pub fn handle_server_events(
    mut server_events: EventReader<RenetServerEvent>,
    mut server: ResMut<RenetServer>,
    init_data: Res<resources::InitData>,
    mut app_state: ResMut<State<resources::GameState>>,
    mut lobby: ResMut<resources::Lobby>,
    mut unassigned_masses: ResMut<UnassignedMasses>,
    physics_config: Res<physics::PhysicsConfig>,
) {
    for event in server_events.iter() {
        match event {
            RenetServerEvent::ClientConnected(id, user_data) => {
                // FIXME: where? here? somewhere we need to "handle" clients connecting
                // to an in-progress game (which we do not allow).
                let new_id = *id;

                let client_preferences = resources::ClientPreferences::from_user_data(user_data);
                debug!("Server got connection from new client {new_id} with preferences {client_preferences:?}");

                let inhabited_mass_id = if let Some(id) = unassigned_masses.0.pop() {
                    id
                } else {
                    warn!("Got connection from client {new_id} but we have no more uninhabited masses");
                    // FIXME: send "error" back to client?
                    return;
                };

                debug!("  sending initial data to client {new_id}");
                let message =
                    bincode::serialize(&events::ToClient::Init(init_data.clone())).unwrap();
                server.send_message(new_id, CHANNEL_RELIABLE, message);

                debug!("  sending physics config to {new_id}");
                let message =
                    bincode::serialize(&events::ToClient::SetPhysicsConfig(*physics_config))
                        .unwrap();
                server.send_message(new_id, CHANNEL_RELIABLE, message);

                debug!("  replaying existing lobby back to new client {new_id:?}");
                for (&existing_id, &client_data) in lobby.clients.iter() {
                    let message = events::ToClient::ClientJoined {
                        id: existing_id,
                        client_data,
                    };
                    server.send_message(
                        new_id,
                        DefaultChannel::Reliable,
                        bincode::serialize(&message).unwrap(),
                    );
                }

                let client_data = resources::ClientData {
                    preferences: client_preferences,
                    inhabited_mass_id,
                };

                debug!("  now updating my lobby with ({new_id}, {client_data:?})");
                lobby.clients.insert(new_id, client_data);
                debug!("  the server now has lobby {lobby:?}");
                let message = events::ToClient::ClientJoined {
                    id: new_id,
                    client_data,
                };
                debug!("  broadcasting about new client (except to {new_id}): {message:?}");
                server.broadcast_message(
                    DefaultChannel::Reliable,
                    bincode::serialize(&message).unwrap(),
                );
            }
            RenetServerEvent::ClientDisconnected(id) => {
                debug!("Server got disconnect from client {id}");
            }
        }
    }

    for client_id in server.clients_id().into_iter() {
        while let Some(message) = server.receive_message(client_id, CHANNEL_RELIABLE) {
            let message = bincode::deserialize(&message).unwrap();
            debug!("Received message from client: {message:?}");
            match message {
                events::ToServer::Ready => {
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
                        resources::GameState::Running
                    } else {
                        resources::GameState::Waiting
                    };
                    let set_state = events::ToClient::SetGameState(state);
                    let message = bincode::serialize(&set_state).unwrap();
                    if start {
                        debug!("Broadcasting {set_state:?}");
                        server.broadcast_message(CHANNEL_RELIABLE, message);
                    } else {
                        // FIXME: we have inconsistency/arbitrariness in 2nd arg choice (channel)
                        debug!("Replying to client {client_id} with {set_state:?}");
                        server.send_message(client_id, DefaultChannel::Reliable, message);
                    }
                    debug!("  and setting my state to {state:?}");
                    let _ = app_state.overwrite_set(state);
                }
                events::ToServer::Rotation(rotation) => {
                    debug!("Sending rotation event for client {client_id}");
                    let client_rotation = events::ToClient::ClientRotation {
                        id: client_id,
                        rotation,
                    };
                    let message = bincode::serialize(&client_rotation).unwrap();
                    debug!("Broadcasting except to {client_id}: {client_rotation:?}");
                    server.broadcast_message_except(client_id, CHANNEL_RELIABLE, message);
                }
                events::ToServer::ProjectileFired(projectile_flight) => {
                    let projectile_fired =
                        events::ToClient::ProjectileFired(projectile_flight);
                    let message = bincode::serialize(&projectile_fired).unwrap();
                    debug!("Broadcasting {projectile_fired:?}");
                    server.broadcast_message(CHANNEL_RELIABLE, message);
                }
            }
        }
    }
}
