use bevy::prelude::*;
use bevy_renet::{
    renet::{RenetClient, RenetError},
    run_if_client_connected, RenetClientPlugin,
};

use mass_gathering::{
    menu_frame, spawn_server_view_camera, systems::spawn_planet, ClientChannel, ClientMessages,
    FullGame, GameState, PhysicsConfig, ServerChannel, ServerMessages,
};

fn main() {
    App::new()
        .add_event::<ClientMessages>()
        .insert_resource(PhysicsConfig { sims_per_frame: 5 })
        .add_plugins(FullGame)
        .add_startup_system(spawn_server_view_camera)
        .add_plugin(RenetClientPlugin::default())
        .add_system(client_sync_players.with_run_criteria(run_if_client_connected))
        .add_system(send_client_messages.with_run_criteria(run_if_client_connected))
        .add_system(panic_on_error_system)
        .add_system_set(SystemSet::on_update(GameState::Stopped).with_system(menu_frame))
        .run();
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
    mut client_messages: EventWriter<ClientMessages>,
    mut app_state: ResMut<State<GameState>>,
) {
    while let Some(message) = client.receive_message(ServerChannel::ServerMessages) {
        let server_message = bincode::deserialize(&message).unwrap();
        match server_message {
            ServerMessages::Init(init_data) => {
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
                let message = ClientMessages::Ready;
                info!("  sending message to server `{message:?}`");
                client_messages.send(message);
            }
            ServerMessages::SetGameState(game_state) => {
                info!("Server says set state to {game_state:?}");
                let _ = app_state.overwrite_set(game_state);
            }
        }
    }
}

fn send_client_messages(
    mut client_messages: EventReader<ClientMessages>,
    mut client: ResMut<RenetClient>,
) {
    for command in client_messages.iter() {
        let message = bincode::serialize(command).unwrap();
        client.send_message(ClientChannel::ClientMessages, message);
    }
}
