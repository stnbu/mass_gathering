use crate::*;
use bevy_egui::{
    egui::{style::Margin, Color32, FontFamily::Monospace, FontId, Frame, RichText, SidePanel},
    EguiContext,
};
use bevy_rapier3d::prelude::{QueryFilter, RapierContext};
use bevy_renet::{
    renet::{ClientAuthentication, RenetClient, RenetConnectionConfig},
    run_if_client_connected, RenetClientPlugin,
};
use std::{net::UdpSocket, time::SystemTime};

const FRAME_FILL: Color32 = Color32::TRANSPARENT;
const TEXT_COLOR: Color32 = Color32::from_rgba_premultiplied(0, 255, 0, 100);

pub fn send_messages_to_server(
    mut client_messages: EventReader<events::ClientMessage>,
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
    mut game_state: ResMut<State<resources::GameState>>,
    mut mass_to_entity_map: ResMut<resources::MassIDToEntity>,
    mut inhabitable_masses: Query<&mut Transform, With<components::Inhabitable>>,
    mut server_messages: EventReader<events::ServerMessage>,
    mut client_messages: EventWriter<events::ClientMessage>,
    mut lobby: ResMut<resources::Lobby>,
    client: Res<RenetClient>,
) {
    let my_id = client.client_id();
    for message in server_messages.iter() {
        debug!("Message for {my_id}");
        match message {
            events::ServerMessage::Init(init_data) => {
                debug!("  got `Init`. Initializing with data receveid from server: {init_data:?}");
                // FIXME: so much clone
                *mass_to_entity_map = init_data
                    .clone()
                    .init(&mut commands, &mut meshes, &mut materials)
                    .clone();
                let message = events::ClientMessage::Ready;
                debug!("  enqueuing message for server `{message:?}`");
                client_messages.send(message);
            }
            events::ServerMessage::SetGameState(new_game_state) => {
                debug!("  got `SetGameState`. Setting state to {new_game_state:?}");
                let _ = game_state.overwrite_set(*new_game_state);
            }
            events::ServerMessage::SetPhysicsConfig(physics_config) => {
                debug!("  got `SetPhysicsConfig`. Inserting resource received from server: {physics_config:?}");
                commands.insert_resource(*physics_config);
            }
            events::ServerMessage::ClientRotation { id, rotation } => {
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
            events::ServerMessage::ClientJoined { id, client_data } => {
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
                    inhabited_mass_commands.insert(components::ClientInhabited);
                    inhabited_mass_commands.remove::<components::Inhabitable>();
                    // FIXME -- Figure out rapier `QueryFilter` so we don't need this (or do we?)
                    inhabited_mass_commands.remove::<RigidBody>();
                    inhabited_mass_commands.despawn_descendants();
                    debug!("    appending camera to inhabited mass to this entity");
                    inhabited_mass_commands.with_children(|child| {
                        child.spawn(Camera3dBundle::default());
                        debug!("    Adding \"sights\"");
                        child.spawn(PbrBundle {
                            mesh: meshes.add(Mesh::from(shape::Icosphere {
                                radius: 0.01,
                                ..Default::default()
                            })),
                            material: materials.add(Color::WHITE.into()),
                            transform: Transform::from_xyz(0.0, 0.0, -2.0),
                            ..Default::default()
                        });
                    });
                }
                debug!("    we now have lobby {lobby:?}");
            }
        }
    }
}

use bevy_rapier3d::prelude::RigidBody;

pub fn receive_messages_from_server(
    mut client: ResMut<RenetClient>,
    mut server_messages: EventWriter<events::ServerMessage>,
) {
    while let Some(message) = client.receive_message(CHANNEL) {
        server_messages.send(bincode::deserialize(&message).unwrap());
    }
}

pub fn new_renet_client(
    client_id: u64,
    client_preferences: resources::ClientPreferences,
    address: String,
) -> RenetClient {
    let address = if let Ok(address) = format!("{address}").parse() {
        address
    } else {
        panic!("Cannot parse address `{address}`");
    };
    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    let authentication = ClientAuthentication::Unsecure {
        client_id,
        protocol_id: PROTOCOL_ID,
        server_addr: address,
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

pub struct ClientPlugin;
impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(Core);
        app.insert_resource(resources::Lobby::default());
        app.add_plugin(Spacetime);
        app.add_system_set(
            SystemSet::on_update(resources::GameState::Waiting)
                .with_system(client::client_waiting_screen),
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

pub fn control(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut client_messages: EventWriter<events::ClientMessage>,
) {
    let nudge = TAU / 10000.0;
    let keys_scaling = 10.0;

    // rotation about local axes
    let mut rotation = Vec3::ZERO;

    // IDEAR: we could just get key counts as f32 and multiply by nudge.
    //   A -> [0, 0, 1]
    //   D -> [0, 0, -1]
    // ...etc
    for key in keys.get_pressed() {
        match key {
            KeyCode::A => {
                rotation.y += nudge;
            }
            KeyCode::D => {
                rotation.y -= nudge;
            }
            KeyCode::W => {
                rotation.z -= nudge;
            }
            KeyCode::S => {
                rotation.z += nudge;
            }
            KeyCode::Z => {
                rotation.x += nudge;
            }
            KeyCode::X => {
                rotation.x -= nudge;
            }
            _ => (),
        }
    }

    if rotation.length() > 0.0000001 {
        let frame_time = time.delta_seconds() * 60.0;
        let [x, y, z] = (rotation * keys_scaling * frame_time).to_array();
        let rotation = Quat::from_euler(EulerRot::XYZ, x, y, z);

        let message = events::ClientMessage::Rotation(rotation);
        client_messages.send(message);
    }
}

// Rotate ME by reading local Rotation events, independant of client/server.
pub fn rotate_client_inhabited_mass(
    mut client_messages: EventReader<events::ClientMessage>,
    mut inhabitant_query: Query<&mut Transform, With<components::ClientInhabited>>,
) {
    if let Ok(mut transform) = inhabitant_query.get_single_mut() {
        for message in client_messages.iter() {
            if let events::ClientMessage::Rotation(rotation) = message {
                transform.rotate(*rotation);
            }
        }
    } else {
        debug!("ClientInhabited entity not present");
    }
}

pub fn client_waiting_screen(mut ctx: ResMut<EguiContext>, lobby: Res<resources::Lobby>) {
    SidePanel::left("client_waiting_screen")
        .resizable(false)
        .min_width(250.0)
        .frame(Frame {
            outer_margin: Margin::symmetric(10.0, 20.0),
            fill: FRAME_FILL,
            ..Default::default()
        })
        .show(ctx.ctx_mut(), |ui| {
            ui.label(
                RichText::new("Waiting for players...")
                    .color(TEXT_COLOR)
                    .font(FontId {
                        size: 20.0,
                        family: Monospace,
                    }),
            );
            ui.separator();
            for (&id, &client_data) in lobby.clients.iter() {
                let nick = to_nick(id);
                let pad = String::from_iter((0..(8 - nick.len())).map(|_| ' '));
                let autostart = if client_data.preferences.autostart {
                    "autostart"
                } else {
                    "wait"
                };
                let text = format!("{nick}{pad}>  {autostart}");
                ui.label(RichText::new(text).color(TEXT_COLOR).font(FontId {
                    size: 16.0,
                    family: Monospace,
                }));
            }
        });
}

#[derive(Debug)]
pub struct HotMass {
    pub mass: Entity,
    pub local_direction: Vec3,
}

pub fn print_the_hot_mass_events(mut hot_mass_events: EventReader<HotMass>) {
    for event in hot_mass_events.iter() {
        warn!("HME: {event:?}");
    }
}

pub fn send_hot_mass_event(
    mass_query: Query<&Transform, (With<components::MassID>, Without<components::Inhabitable>)>,
    inhabited_mass_query: Query<&Transform, With<components::ClientInhabited>>,
    rapier_context: Res<RapierContext>,
    mut hot_mass_events: EventWriter<HotMass>,
) {
    for client_pov in inhabited_mass_query.iter() {
        let ray_origin = client_pov.translation;
        let ray_direction = -1.0 * client_pov.local_z();
        let intersection = rapier_context.cast_ray(
            ray_origin,
            ray_direction,
            150.0,
            false,
            QueryFilter::only_dynamic(),
        );
        if let Some((mass, distance)) = intersection {
            if let Ok(mass_transform) = mass_query.get(mass) {
                let global_impact_site = ray_origin + (ray_direction * distance);
                let local_direction = (global_impact_site - mass_transform.translation).normalize();
                let event = HotMass {
                    mass,
                    local_direction,
                };
                hot_mass_events.send(event);
            }
        }
    }
}
