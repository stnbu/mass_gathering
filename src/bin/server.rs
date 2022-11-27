use bevy::log::LogPlugin;
use local_ip_address::local_ip;
use mass_gathering::*;
use std::{
    net::{SocketAddr, UdpSocket},
    time::SystemTime,
};

fn create_renet_server() -> RenetServer {
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();

    let server_addr = SocketAddr::new(local_ip().unwrap(), PORT_NUMBER);

    let server_config =
        ServerConfig::new(64, PROTOCOL_ID, server_addr, ServerAuthentication::Unsecure);

    let connection_config = RenetConnectionConfig::default();

    let inbound_server_addr = SocketAddr::new(local_ip().unwrap(), PORT_NUMBER);
    let socket = UdpSocket::bind(inbound_server_addr).unwrap();

    RenetServer::new(current_time, server_config, connection_config, socket).unwrap()
}

fn main() {
    let server_resource = create_renet_server();

    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugin(LogPlugin {
            filter: "info,wgpu_core=warn,wgpu_hal=off,rechannel=warn".into(),
            level: bevy::log::Level::DEBUG,
        })
        .add_plugin(RenetServerPlugin::default())
        .add_system(server_events)
        .add_system(serve)
        .insert_resource(server_resource)
        .run();

    //info!("Adding server resource {server_resource:?}");
}

fn server_events(mut events: EventReader<ServerEvent>) {
    for event in events.iter() {
        info!("Event: {event:?}");
    }
}

fn serve(mut server: ResMut<RenetServer>) {
    let reliable_channel_id = ReliableChannelConfig::default().channel_id;

    for client_id in server.clients_id().into_iter() {
        while let Some(message) = server.receive_message(client_id, reliable_channel_id) {
            let client_message = bincode::deserialize(&message).unwrap();
            match client_message {
                ClientMessage::StateBroadcast { translation } => {
                    info!("Got translation of {translation:?} from {client_id}");
                    let inverse = ServerMessage::RelayedBroadcast {
                        translation: -translation,
                    };
                    info!(" ...sending back inverse which is {inverse:?}");
                    server.send_message(
                        client_id,
                        reliable_channel_id,
                        bincode::serialize(&inverse).unwrap(),
                    );
                }
            }
        }
    }
}
