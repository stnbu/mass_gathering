use std::{net::UdpSocket, time::SystemTime};

use bevy::prelude::*;

use bevy_renet::{
    renet::{ClientAuthentication, RenetClient, RenetError},
    run_if_client_connected, RenetClientPlugin,
};

use mass_gathering::{
    client_connection_config, spawn_server_view_camera, systems::spawn_planet, ServerChannel,
    ServerMessages, PORT_NUMBER, PROTOCOL_ID, SERVER_ADDR,
};

fn new_renet_client() -> RenetClient {
    let client_id = 0;
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
    app.insert_resource(new_renet_client());
    app.add_startup_system(spawn_server_view_camera);
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
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut client: ResMut<RenetClient>,
) {
    while let Some(message) = client.receive_message(ServerChannel::ServerMessages) {
        let server_message = bincode::deserialize(&message).unwrap();
        match server_message {
            ServerMessages::SendInitData(init_data) => {
                info!(
                    "Server sent init data for {} planets to client {}",
                    init_data.planets.len(),
                    client.client_id()
                );
                info!("  spawning planets...");
                for (&planet_id, &planet_init_data) in init_data.planets.iter() {
                    spawn_planet(
                        planet_id,
                        planet_init_data,
                        &mut commands,
                        &mut meshes,
                        &mut materials,
                    );
                }
                //
            }
        }
    }
}
