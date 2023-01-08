use bevy::prelude::*;
use bevy_renet::renet::{ClientAuthentication, RenetClient, RenetConnectionConfig};
use std::collections::HashSet;
use std::{net::UdpSocket, time::SystemTime};

use crate::{
    inhabitant::{ClientInhabited, Inhabitable},
    networking::*,
    GameState, MassIDToEntity,
};

#[derive(Default)]
struct InhabitableTaken(HashSet<u64>);

pub fn send_messages_to_server(
    mut client_messages: EventReader<ClientMessages>,
    mut client: ResMut<RenetClient>,
) {
    for message in client_messages.iter() {
        client.send_message(CHANNEL, bincode::serialize(message).unwrap());
    }
}

pub fn process_server_messages(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut game_state: ResMut<State<GameState>>,
    mut mass_to_entity_map: ResMut<MassIDToEntity>,
    mut inhabitable_masses: Query<&mut Transform, With<Inhabitable>>,
    mut server_messages: EventReader<ServerMessage>,
    mut client_messages: EventWriter<ClientMessages>,
    mut lobby: ResMut<Lobby>,
) {
    for message in server_messages.iter() {
        match message {
            ServerMessage::Init(init_data) => {
                debug!("Initializing with data receveid from server: {init_data:?}");
                // FIXME: so much clone
                *mass_to_entity_map = init_data
                    .clone()
                    .init(&mut commands, &mut meshes, &mut materials)
                    .clone();
                let message = ClientMessages::Ready;
                debug!("  enqueuing message for server `{message:?}`");
                client_messages.send(message);
            }
            ServerMessage::SetGameState(new_game_state) => {
                debug!("Server says set state to {game_state:?}. Setting state now.");
                let _ = game_state.overwrite_set(*new_game_state);
            }
            ServerMessage::SetPhysicsConfig(physics_config) => {
                debug!("Inserting resource received from server: {physics_config:?}");
                commands.insert_resource(*physics_config);
            }
            ServerMessage::ClientRotation { id, rotation } => {
                let mass_id = lobby.clients.get(id).unwrap().inhabited_mass_id;
                if let Some(entity) = mass_to_entity_map.0.get(&mass_id) {
                    if let Ok(mut mass_transform) = inhabitable_masses.get_mut(*entity) {
                        debug!("Got rotate event for {id} corresponding to entity {entity:?}");
                        mass_transform.rotate(*rotation);
                    } else {
                        error!("Entity map for mass ID {id} as entity {entity:?} which does not exist.");
                    }
                } else {
                    error!(
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
                if let Some(old) = lobby.clients.insert(*id, *client_data) {
                    debug!("  the value {old:?} was replaced for client {id}");
                }
                debug!("  we now has lobby {lobby:?}");
            }
        }
    }
}

pub fn receive_messages_from_server(
    mut client: ResMut<RenetClient>,
    mut server_messages: EventWriter<ServerMessage>,
) {
    while let Some(message) = client.receive_message(CHANNEL) {
        server_messages.send(bincode::deserialize(&message).unwrap());
    }
}

pub fn new_renet_client(client_id: u64, client_preferences: ClientPreferences) -> RenetClient {
    let server_addr = format!("{SERVER_ADDR}:{PORT_NUMBER}").parse().unwrap();
    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    let authentication = ClientAuthentication::Unsecure {
        client_id,
        protocol_id: PROTOCOL_ID,
        server_addr,
        user_data: Some(client_preferences.to_netcode_user_data()),
    };
    RenetClient::new(
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap(),
        socket,
        RenetConnectionConfig::default(),
        authentication,
    )
    .unwrap()
}
