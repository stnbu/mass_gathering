use local_ip_address::local_ip;
use mass_gathering::*;
use std::{
    net::{SocketAddr, UdpSocket},
    time::SystemTime,
};

fn create_renet_client() -> RenetClient {
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();

    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();

    let client_id = current_time.as_millis() as u64;

    let connection_config = RenetConnectionConfig::default();

    //TODO Prompt for server IP
    let server_addr = SocketAddr::new(local_ip().unwrap(), PORT_NUMBER);

    let authentication = ClientAuthentication::Unsecure {
        client_id,
        protocol_id: PROTOCOL_ID,
        server_addr,
        user_data: None,
    };

    RenetClient::new(current_time, socket, connection_config, authentication).unwrap()
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(bevy::log::LogPlugin {
                    filter: "info,wgpu_core=warn,wgpu_hal=off,rechannel=warn".into(),
                    level: bevy::log::Level::DEBUG,
                })
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        width: 1280.,
                        height: 720.,
                        title: "Renet Demo Client".to_string(),
                        resizable: false,
                        ..Default::default()
                    },
                    ..Default::default()
                }),
        )
        .add_plugin(RenetClientPlugin::default())
        .insert_resource(create_renet_client())
        .add_system(send_receive)
        .add_system(bevy::window::close_on_esc)
        .run();
}

fn send_receive(mut client: ResMut<RenetClient>, keyboard: Res<Input<KeyCode>>) {
    let reliable_channel_id = ReliableChannelConfig::default().channel_id;

    let message = ClientMessage::StateBroadcast {
        translation: Vec3::ZERO,
    };

    if keyboard.just_pressed(KeyCode::Space) {
        client.send_message(reliable_channel_id, bincode::serialize(&message).unwrap());
        info!("Sent state broadcast: {message:?}");
    }

    while let Some(message) = client.receive_message(reliable_channel_id) {
        match bincode::deserialize(&message).unwrap() {
            ServerMessage::RelayedBroadcast { translation } => {
                info!("Got thuh broadcast response from thuh server: {translation:?}");
            }
        }
    }
}
