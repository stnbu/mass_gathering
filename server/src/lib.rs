use bevy::app::AppExit;
use bevy_renet::renet::ServerEvent;
use bevy_renet::renet::{
    DefaultChannel, RenetConnectionConfig, RenetServer, ServerAuthentication, ServerConfig,
};
use game::*;

use std::{net::UdpSocket, time::SystemTime};

pub mod plugins;

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

pub fn setup_game(
    mut commands: Commands,
    cli_args: Res<resources::ServerCliArgs>,
    mut game_startup_data: ResMut<resources::GameStartupData>,
) {
    let speed = cli_args.speed;
    let zerog = cli_args.zerog;
    let init_data = systems::get_system(&cli_args.system)();
    for (mass_id, mass_init_data) in init_data.masses.iter() {
        if mass_init_data.inhabitable {
            game_startup_data.unassigned_mass_ids.push(*mass_id);
        }
    }
    commands.insert_resource(resources::GameConfig {
        physics_config: resources::PhysicsConfig { speed, zerog },
        init_data,
        ..Default::default()
    });
}

pub fn handle_server_events(
    mut server_events: EventReader<ServerEvent>,
    mut server: ResMut<RenetServer>,
    mut app_state: ResMut<State<resources::GameState>>,
    mut game_config: ResMut<resources::GameConfig>,
    mut game_startup_data: ResMut<resources::GameStartupData>,
    mut exit: EventWriter<AppExit>,
) {
    for event in server_events.iter() {
        match event {
            &ServerEvent::ClientConnected(id, _) => {
                if let Some(mass_id) = game_startup_data.unassigned_mass_ids.pop() {
                    game_config.client_mass_map.insert(id, mass_id);
                    if game_startup_data.unassigned_mass_ids.is_empty() {
                        server.broadcast_message(
                            DefaultChannel::Reliable,
                            bincode::serialize(&events::ToClient::SetGameConfig(
                                game_config.clone(),
                            ))
                            .unwrap(),
                        );
                    }
                };
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
                events::ToServer::Ready => {
                    if game_startup_data.unassigned_mass_ids.is_empty() {
                        let state = resources::GameState::Running;
                        let set_state = events::ToClient::SetGameState(state);
                        let message = bincode::serialize(&set_state).unwrap();
                        // FIXME: This could occur before we've received all "Ready"s.
                        // We might have to keep our "Ready count" separately to if it matters.
                        server.broadcast_message(DefaultChannel::Reliable, message);
                        let _ = app_state.overwrite_set(state);
                    } else {
                        let state = resources::GameState::Waiting;
                        let set_state = events::ToClient::SetGameState(state);
                        let message = bincode::serialize(&set_state).unwrap();
                        server.send_message(client_id, DefaultChannel::Reliable, message);
                        let _ = app_state.overwrite_set(state);
                    }
                }
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
