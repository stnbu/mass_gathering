use std::{collections::HashMap, net::UdpSocket, time::SystemTime};

use bevy::prelude::*;

use bevy_renet::{
    renet::{ClientAuthentication, RenetClient, RenetError},
    run_if_client_connected, RenetClientPlugin,
};
use clap::Parser;
use mass_gathering::{
    client_connection_config, NetworkedEntities, ServerChannel, ServerMessages, PORT_NUMBER,
    PROTOCOL_ID, SERVER_ADDR,
};

#[derive(Component)]
struct ControlledPlayer;

#[derive(Default, Resource)]
struct NetworkMapping(HashMap<Entity, Entity>);

#[derive(Debug)]
struct PlayerInfo {
    id: u64,
}

#[derive(Debug, Default, Resource)]
struct ClientLobby {
    players: HashMap<u64, PlayerInfo>,
}

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    id: u64,
}

fn new_renet_client() -> RenetClient {
    let client_id = Args::parse().id;
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
        user_data: None,
    };

    RenetClient::new(current_time, socket, connection_config, authentication).unwrap()
}

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugin(RenetClientPlugin::default());
    app.insert_resource(ClientLobby::default());
    app.insert_resource(new_renet_client());
    app.insert_resource(NetworkMapping::default());

    app.add_system(client_sync_players.with_run_criteria(run_if_client_connected));
    app.add_system(panic_on_error_system);

    app.run();
}

fn panic_on_error_system(mut renet_error: EventReader<RenetError>) {
    for e in renet_error.iter() {
        panic!("{}", e);
    }
}

fn client_sync_players(
    mut client: ResMut<RenetClient>,
    mut lobby: ResMut<ClientLobby>,
    mut network_mapping: ResMut<NetworkMapping>,
) {
    let _my_id = client.client_id(); // is it "my"?
    while let Some(message) = client.receive_message(ServerChannel::ServerMessages) {
        let server_message = bincode::deserialize(&message).unwrap();
        match server_message {
            ServerMessages::PlayerCreate { id } => {
                println!("Player {} connected.", id);
                let player_info = PlayerInfo { id };
                lobby.players.insert(id, player_info);
            }
            ServerMessages::PlayerRemove { id } => {
                println!("Player {} disconnected.", id);
            }
        }
    }
    while let Some(message) = client.receive_message(ServerChannel::NetworkedEntities) {
        let networked_entities: NetworkedEntities = bincode::deserialize(&message).unwrap();
        for i in 0..networked_entities.entities.len() {
            if let Some(_entity) = network_mapping.0.get(&networked_entities.entities[i]) {
                //
            }
        }
    }
}
