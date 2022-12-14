use bevy::prelude::*;
use bevy_renet::renet::{ClientAuthentication, RenetClient, RenetConnectionConfig};
use std::collections::HashSet;
use std::{net::UdpSocket, time::SystemTime};

use crate::{networking::*, systems::spawn_mass, GameConfig, GameState};

#[derive(Component)]
pub struct Inhabited;

#[derive(Component)]
pub struct Inhabitable;

#[derive(Default)]
struct InhabitableTaken(HashSet<u64>);

pub fn handle_client_events(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut client: ResMut<RenetClient>,
    mut client_messages: EventWriter<ClientMessages>,
    mut game_state: ResMut<State<GameState>>,
    mut mass_to_entity_map: ResMut<MapMassIDToEntity>,
    mut inhabitable_masses: Query<&mut Transform, With<Inhabitable>>,
    mut lobby: ResMut<Lobby>,
) {
    while let Some(message) = client.receive_message(ServerChannel::ServerMessages) {
        let server_message = bincode::deserialize(&message).unwrap();
        match server_message {
            ServerMessages::Init(init_data) => {
                debug!(
                    "Server sent init data for {} uninhabitable and {} inhabitable masses to me, client {}",
                    init_data.uninhabitable_masses.len(),
                    init_data.inhabitable_masses.len(),
                    client.client_id()
                );
                debug!("  spawning masses...");
                for (mass_id, mass_init_data) in init_data.uninhabitable_masses.iter() {
                    let mass_entity = spawn_mass(
                        false,
                        *mass_id,
                        *mass_init_data,
                        &mut commands,
                        &mut meshes,
                        &mut materials,
                    );
                    mass_to_entity_map.0.insert(*mass_id, mass_entity);
                }
                for (mass_id, mass_init_data) in init_data.inhabitable_masses.iter() {
                    let mass_entity = spawn_mass(
                        true,
                        *mass_id,
                        *mass_init_data,
                        &mut commands,
                        &mut meshes,
                        &mut materials,
                    );
                    don_inhabitant_garb(mass_entity, &mut commands, &mut meshes, &mut materials);
                    mass_to_entity_map.0.insert(*mass_id, mass_entity);
                }
                let message = ClientMessages::Ready;
                debug!("  sending message to server `{message:?}`");
                client_messages.send(message);
            }
            ServerMessages::SetGameState(new_game_state) => {
                // FIXME: Why in the _whorld_ would we receive this? --
                // "Server says set state to ResMut(
                //     State { transition: None, stack: [Stopped], scheduled: None, end_next_loop: false }
                // ). Setting state now."
                debug!("Server says set state to {game_state:?}. Setting state now.");
                let _ = game_state.overwrite_set(new_game_state);
            }
            ServerMessages::SetPhysicsConfig(physics_config) => {
                debug!("Inserting resource received from server: {physics_config:?}");
                commands.insert_resource(physics_config);
            }
            ServerMessages::ClientRotation { id, rotation } => {
                let mass_id = lobby.clients.get(&id).unwrap().inhabited_mass_id;
                if let Some(entity) = mass_to_entity_map.0.get(&mass_id) {
                    if let Ok(mut mass_transform) = inhabitable_masses.get_mut(*entity) {
                        debug!("Rotating inhabitable mass {id} to {rotation}");
                        mass_transform.rotation = rotation;
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
            ServerMessages::ClientJoined { id, client_data } => {
                debug!(
                    "Server says ({}, {:?}) connected. Updating my lobby.",
                    id, client_data
                );
                if id == client.client_id() {
                    debug!("  fyi, that's me (I am {id})");
                    let inhabited_mass = mass_to_entity_map
                        .0
                        .get(&client_data.inhabited_mass_id)
                        .unwrap();
                    debug!("  found exactly one mass for me to inhabit: {inhabited_mass:?}");
                    let mut inhabited_mass_commands = commands.entity(*inhabited_mass);
                    inhabited_mass_commands.insert(Inhabited);
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

pub fn set_window_title(
    game_state: Res<State<GameState>>,
    mut windows: ResMut<Windows>,
    game_config: Res<GameConfig>,
) {
    let nick = if game_config.nickname.is_empty() {
        "<unset>"
    } else {
        &game_config.nickname
    };
    let window = windows.primary_mut();
    window.set_title(format!("Client[{:?}] : nick={nick}", game_state.current()));
}

pub fn control(
    keys: Res<Input<KeyCode>>,
    mut inhabitant_query: Query<&mut Transform, With<Inhabited>>,
    time: Res<Time>,
    mut client_messages: EventWriter<ClientMessages>,
) {
    let mut transform = inhabitant_query
        .get_single_mut()
        .expect("Could not get transform of `Inhabited` entity");

    let nudge = TAU / 10000.0;
    let keys_scaling = 10.0;

    // rotation about local axes
    let mut rotation = Vec3::ZERO;

    for key in keys.get_pressed() {
        match key {
            KeyCode::A => {
                rotation.y += nudge;
            }
            KeyCode::D => {
                rotation.y -= nudge;
            }
            KeyCode::W => {
                rotation.x += nudge;
            }
            KeyCode::S => {
                rotation.x -= nudge;
            }
            KeyCode::Z => {
                rotation.z += nudge;
            }
            KeyCode::X => {
                rotation.z -= nudge;
            }
            _ => (),
        }
    }

    if rotation.length() > 0.0000001 {
        let frame_time = time.delta_seconds() * 60.0;
        rotation *= keys_scaling * frame_time;
        let local_x = transform.local_x();
        let local_y = transform.local_y();
        let local_z = transform.local_z();
        transform.rotate(Quat::from_axis_angle(local_x, rotation.x));
        transform.rotate(Quat::from_axis_angle(local_z, rotation.z));
        transform.rotate(Quat::from_axis_angle(local_y, rotation.y));

        let message = ClientMessages::Rotation(transform.rotation);
        debug!("  sending message to server `{message:?}`");
        client_messages.send(message);
    }
}
