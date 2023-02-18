/// Server stuff. Will/may include both "simulation" and "visual" stuff (for debugging).
///
/// But in this case there will be exactly zero instead of exactly one inhabited mass, should be optionally headless, and "reads its own output", i.e. anything that gets sent to clients also gets sent to this server's internal simulation (rotation does not matter). And it should be `x,y,z,v,m` exactly the same as the clients, or close enough.
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
/// refactor_tags: UNSET
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

/// refactor_tags: UNSET
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

/// refactor_tags: UNSET
pub fn handle_server_events(
    mut server_events: EventReader<ServerEvent>,
    mut server: ResMut<RenetServer>,
    mut app_state: ResMut<State<resources::GameState>>,
    mut game_config: ResMut<resources::GameConfig>,
    mut exit: EventWriter<AppExit>,
) {
    for event in server_events.iter() {
        match *event {
            ServerEvent::ClientConnected(client_id, _) => match game_config.get_free_id() {
                Ok(mass_id) => {
                    debug!("Connection from client {client_id} assigning mass {mass_id}");
                    // FIXME: Now we are keeping client_id->mass_id mapping in two places.
                    // Several kinds of yuck.
                    // FIXME: Also the client does this same thing on the other end, directly from CLI args. Do we "ship" this also?
                    let player = components::Player::from_id(client_id);
                    game_config
                        .init_data
                        .masses
                        .get_mut(&mass_id)
                        .unwrap()
                        .inhabitation = components::Inhabitation::Inhabitable(Some(player));
                    match game_config.client_mass_map.insert(client_id, mass_id) {
                        None => {
                            if game_config.is_capacity() {
                                debug!("Game is now at capacity. Set state Running");
                                server.broadcast_message(
                                    DefaultChannel::Reliable,
                                    bincode::serialize(&events::ToClient::SetGameConfig(
                                        game_config.clone(),
                                    ))
                                    .unwrap(),
                                );
                                if let Err(err) =
                                    app_state.overwrite_set(resources::GameState::Running)
                                {
                                    panic!("When setting state to Running: {err:?}");
                                }
                                server.broadcast_message(
                                    DefaultChannel::Reliable,
                                    bincode::serialize(&events::ToClient::SetGameState(
                                        resources::GameState::Running,
                                    ))
                                    .unwrap(),
                                );
                            } else {
                                debug!("Game is not at capacity. Set state Waiting");
                                server.broadcast_message(
                                    DefaultChannel::Reliable,
                                    bincode::serialize(&events::ToClient::SetGameState(
                                        resources::GameState::Waiting,
                                    ))
                                    .unwrap(),
                                );
                                let _ = app_state.overwrite_set(resources::GameState::Waiting);
                            }
                        }
                        Some(id) => {
                            error!("Client already assigigned mass {id}");
                        }
                    }
                }
                Err(err) => {
                    error!("While getting free mass id: {err}");
                }
            },
            ServerEvent::ClientDisconnected(client_id) => {
                warn!("Client {client_id} has disconnected");
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
                    debug!("Broadcasting projectile flight");
                    server.broadcast_message(DefaultChannel::Reliable, message);
                }
            }
        }
    }
}
