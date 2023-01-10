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
    mut client_messages: EventReader<ClientMessage>,
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
    mut client_messages: EventWriter<ClientMessage>,
    mut lobby: ResMut<Lobby>,
    client: Res<RenetClient>,
) {
    let my_id = client.client_id();
    for message in server_messages.iter() {
        debug!("Message for {my_id}");
        match message {
            ServerMessage::Init(init_data) => {
                debug!("  got `Init`. Initializing with data receveid from server: {init_data:?}");
                // FIXME: so much clone
                *mass_to_entity_map = init_data
                    .clone()
                    .init(&mut commands, &mut meshes, &mut materials)
                    .clone();
                let message = ClientMessage::Ready;
                debug!("  enqueuing message for server `{message:?}`");
                client_messages.send(message);
            }
            ServerMessage::SetGameState(new_game_state) => {
                debug!("  got `SetGameState`. Setting state to {new_game_state:?}");
                let _ = game_state.overwrite_set(*new_game_state);
            }
            ServerMessage::SetPhysicsConfig(physics_config) => {
                debug!("  got `SetPhysicsConfig`. Inserting resource received from server: {physics_config:?}");
                commands.insert_resource(*physics_config);
            }
            ServerMessage::ClientRotation { id, rotation } => {
                debug!("  got `ClientRotation`. Rotating mass {id}");
                let mass_id = lobby.clients.get(id).unwrap().inhabited_mass_id;
                if let Some(entity) = mass_to_entity_map.0.get(&mass_id) {
                    if let Ok(mut mass_transform) = inhabitable_masses.get_mut(*entity) {
                        debug!("    found corresponding entity {entity:?}");
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
                debug!("  got `ClientJoined`. Inserting entry for client {id}");
                if let Some(old) = lobby.clients.insert(*id, *client_data) {
                    warn!("  the value {old:?} was replaced for client {id}");
                }
                if *id == client.client_id() {
                    let inhabited_mass = mass_to_entity_map
                        .0
                        .get(&client_data.inhabited_mass_id)
                        .unwrap();
                    debug!("    server has assigned to me mass id {} which I map to entity {inhabited_mass:?}",
			   client_data.inhabited_mass_id);
                    let mut inhabited_mass_commands = commands.entity(*inhabited_mass);
                    debug!("    inserting `ClientInhabited` component into this mass entity (meaing 'this is mine')");
                    inhabited_mass_commands.insert(ClientInhabited);
                    inhabited_mass_commands.despawn_descendants();
                    debug!("    appending camera to inhabited mass to this entity");
                    inhabited_mass_commands.with_children(|child| {
                        child.spawn(Camera3dBundle::default());
                    });
                }
                debug!("    we now have lobby {lobby:?}");
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

//

pub struct ClientPlugin;
impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(Core);
        app.insert_resource(Lobby::default());
        app.add_plugin(Spacetime);
        app.add_system_set(
            SystemSet::on_update(GameState::Waiting).with_system(ui::client_waiting_screen),
        );
        app.add_plugin(RenetClientPlugin::default());

        app.add_system(client::send_messages_to_server.with_run_criteria(run_if_client_connected));
        app.add_system(client::process_server_messages.with_run_criteria(run_if_client_connected));
        app.add_system(
            client::receive_messages_from_server.with_run_criteria(run_if_client_connected),
        );
        app.add_system(panic_on_renet_error);
    }
}
