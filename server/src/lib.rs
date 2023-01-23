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
    mut server_events: EventReader<ServerEvent>,
    mut server: ResMut<RenetServer>,
    init_data: Res<resources::InitData>,
    mut app_state: ResMut<State<resources::GameState>>,
    mut lobby: ResMut<resources::Lobby>,
    mut unassigned_masses: ResMut<UnassignedMasses>,
    physics_config: Res<physics::PhysicsConfig>,
    mut exit: EventWriter<AppExit>,
) {
    for event in server_events.iter() {
        match event {
            ServerEvent::ClientConnected(id, _) => {
                let new_client_id = *id;
                let inhabited_mass_id = if let Some(mass_id) = unassigned_masses.0.pop() {
                    mass_id
                } else {
                    warn!("Got connection from client {new_client_id} but we have no more uninhabited masses");
                    // FIXME: send "error" back to client?
                    return;
                };

                debug!("  sending initial data to client {new_client_id}");
                let message =
                    bincode::serialize(&events::ToClient::Init(init_data.clone())).unwrap();
                server.send_message(new_client_id, DefaultChannel::Reliable, message);

                debug!("  sending physics config to {new_client_id}");
                let message =
                    bincode::serialize(&events::ToClient::SetPhysicsConfig(*physics_config))
                        .unwrap();
                server.send_message(new_client_id, DefaultChannel::Reliable, message);

                debug!("  replaying existing lobby back to new client {new_client_id:?}");
                for (&existing_id, &client_data) in lobby.clients.iter() {
                    let message = events::ToClient::ClientJoined {
                        id: existing_id,
                        client_data,
                    };
                    server.send_message(
                        new_client_id,
                        DefaultChannel::Reliable,
                        bincode::serialize(&message).unwrap(),
                    );
                }
                // FIXME: This probably wants to be under "ToServer::Ready"
                let client_data = resources::ClientData { inhabited_mass_id };
                debug!("  now updating my lobby with ({new_client_id}, {client_data:?})");
                lobby.clients.insert(new_client_id, client_data);
                debug!("  the server now has lobby {lobby:?}");
                let message = events::ToClient::ClientJoined {
                    id: new_client_id,
                    client_data,
                };
                debug!("  broadcasting about new client: {message:?}");
                // NOTE: This is how the client gets its own entry in its copy of `Lobby`.
                server.broadcast_message(
                    DefaultChannel::Reliable,
                    bincode::serialize(&message).unwrap(),
                );
            }
            ServerEvent::ClientDisconnected(id) => {
                debug!(
                    "Server got disconnect from client {id} ({}). Quiting Bevy app.",
                    to_nick(*id).trim_end()
                );
                exit.send(AppExit);
            }
        }
    }

    for client_id in server.clients_id().into_iter() {
        // FIXME: What is the "clean" way to "iterate all channels".
        while let Some(message) = server.receive_message(client_id, DefaultChannel::Reliable) {
            let message = bincode::deserialize(&message).unwrap();
            trace!("Received message from client {client_id}: {message:?}");
            match message {
                events::ToServer::Ready => {
                    debug!("Got 'Ready' from {client_id}");
                    let start = lobby.clients.len()
                        == init_data
                            .masses
                            .iter()
                            .filter(|(_, data)| data.inhabitable)
                            .count();
                    if start {
                        /* PROBLEM! (this should print only once)
                        yolo:mass_gathering mburr$ less -R /tmp/logs/1674427249/server.out | grep 'capacity'
                        2023-01-22T22:40:55.953958Z DEBUG server:   game has now reached capacity.
                        2023-01-22T22:40:55.973865Z DEBUG server:   game has now reached capacity.
                        2023-01-22T22:40:56.004610Z DEBUG server:   game has now reached capacity.
                        */
                        debug!("  game has now reached capacity.");
                        let state = resources::GameState::Running;
                        let set_state = events::ToClient::SetGameState(state);
                        let message = bincode::serialize(&set_state).unwrap();
                        debug!("Broadcasting {set_state:?}");
                        server.broadcast_message(DefaultChannel::Reliable, message);
                        debug!("  and setting my state to {state:?}");
                        let _ = app_state.overwrite_set(state);
                    } else {
                        let state = resources::GameState::Waiting;
                        let set_state = events::ToClient::SetGameState(state);
                        let message = bincode::serialize(&set_state).unwrap();
                        debug!("Replying to client {client_id} with {state:?}");
                        server.send_message(client_id, DefaultChannel::Reliable, message);
                        debug!("  and setting my state to {set_state:?}");
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
