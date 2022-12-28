use bevy::prelude::*;
use bevy_renet::renet::{ClientAuthentication, RenetClient, RenetConnectionConfig};
use std::collections::HashSet;
use std::{net::UdpSocket, time::SystemTime};

use crate::{
    inhabitant::{ClientInhabited, ClientRotation, Inhabitable},
    networking::*,
    GameState, MassIDToEntity,
};

#[derive(Default)]
struct InhabitableTaken(HashSet<u64>);

pub fn handle_client_events(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut client: ResMut<RenetClient>,
    mut client_messages: EventWriter<ClientMessages>,
    mut game_state: ResMut<State<GameState>>,
    mut mass_to_entity_map: ResMut<MassIDToEntity>,
    mut inhabitable_masses: Query<&mut Transform, With<Inhabitable>>,
    mut lobby: ResMut<Lobby>,
) {
    while let Some(message) = client.receive_message(ServerChannel::ServerMessages) {
        let server_message = bincode::deserialize(&message).unwrap();
        match server_message {
            ServerMessage::Init(init_data) => {
                debug!("Initializing with data receveid from server: {init_data:?}");
                // FIXME: so much clone
                *mass_to_entity_map = init_data
                    .clone()
                    .init(&mut commands, &mut meshes, &mut materials)
                    .clone();
                let message = ClientMessages::Ready;
                debug!("  sending message to server `{message:?}`");
                client_messages.send(message);
            }
            ServerMessage::SetGameState(new_game_state) => {
                debug!("Server says set state to {game_state:?}. Setting state now.");
                let _ = game_state.overwrite_set(new_game_state);
            }
            ServerMessage::SetPhysicsConfig(physics_config) => {
                debug!("Inserting resource received from server: {physics_config:?}");
                commands.insert_resource(physics_config);
            }
            ServerMessage::ClientRotation { id, rotation } => {
                assert!(
                    id != client.client_id(),
                    "Server sent me my own rotation event."
                );
                let mass_id = lobby.clients.get(&id).unwrap().inhabited_mass_id;
                if let Some(entity) = mass_to_entity_map.0.get(&mass_id) {
                    if let Ok(mut mass_transform) = inhabitable_masses.get_mut(*entity) {
                        debug!("Rotating inhabitable mass {id} by {rotation}");
                        mass_transform.rotate(rotation);
                    } else {
                        println!("query no!");
                    }
                } else {
                    panic!(
                        "Unable to find client {id} in entity mapping {:?}",
                        mass_to_entity_map.0
                    )
                }
            }
            ServerMessage::ClientJoined { id, client_data } => {
                debug!(
                    "Server says ({}, {:?}) connected. Updating my lobby.",
                    id, client_data
                );
                if id == client.client_id() {
                    // FIXME: some logic overlap with setup_standalone
                    debug!("  fyi, that's me (I am {id})");
                    let inhabited_mass = mass_to_entity_map
                        .0
                        .get(&client_data.inhabited_mass_id)
                        .unwrap();
                    debug!("  found exactly one mass for me to inhabit: {inhabited_mass:?}");
                    let mut inhabited_mass_commands = commands.entity(*inhabited_mass);
                    inhabited_mass_commands.insert(ClientInhabited);
                    inhabited_mass_commands.despawn_descendants();
                    debug!("Appending camera to inhabited mass {inhabited_mass:?}");
                    inhabited_mass_commands.with_children(|child| {
                        child.spawn(Camera3dBundle::default());
                    });
                }
                if let Some(old) = lobby.clients.insert(id, client_data) {
                    debug!("  the value {old:?} was replaced for client {id}");
                }
                debug!("  client {} now has lobby {lobby:?}", client.client_id());
            }
        }
    }
}

pub fn send_rotation_to_server(
    mut rotation_events: EventReader<ClientRotation>,
    mut client_messages: EventWriter<ClientMessages>,
) {
    for ClientRotation(rotation) in rotation_events.iter() {
        let message = ClientMessages::Rotation(*rotation);
        debug!("  sending message to server `{message:?}`");
        client_messages.send(message);
    }
}

pub fn send_client_messages(
    mut client_messages: EventReader<ClientMessages>,
    mut client: ResMut<RenetClient>,
) {
    for command in client_messages.iter() {
        let message = bincode::serialize(command).unwrap();
        client.send_message(ClientChannel::ClientMessages, message);
    }
}

pub fn client_connection_config() -> RenetConnectionConfig {
    RenetConnectionConfig {
        receive_channels_config: ServerChannel::channels_config(),
        ..Default::default()
    }
}

pub fn new_renet_client(client_id: u64, client_preferences: ClientPreferences) -> RenetClient {
    let server_addr = format!("{SERVER_ADDR}:{PORT_NUMBER}").parse().unwrap();
    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    let connection_config = client_connection_config();
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    let authentication = ClientAuthentication::Unsecure {
        client_id,
        protocol_id: PROTOCOL_ID,
        server_addr,
        user_data: Some(client_preferences.to_netcode_user_data()),
    };
    RenetClient::new(current_time, socket, connection_config, authentication).unwrap()
}
