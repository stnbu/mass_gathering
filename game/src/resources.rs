use crate::*;
use bevy_rapier3d::prelude::{Collider, RigidBody};
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// FIXME
//
// There's ~~ClientPreferences~~, ClientData ... and plenty
// of other stuff that is a mishmash of hodgepodge. But it's complicated.
// Some thing are "requested" by the client, some things are "assigned"
// by the server. Some things only belong on the client/server, some things
// should never be over-written (write-once, no update)...
//
// It's all sixes and sevens
//
// It's all higgledy piggledy
//
// Can't find its ass in a wet paper barn
//
// [Update]
//
// There is a similar situation with
//
//   ClientCliArgs -> ClientPreferences (not anymore)
//   ServerCliArgs -> PhysicsConfig (and there is also a ServerConfig)

#[derive(Parser, Resource)]
pub struct ClientCliArgs {
    #[arg(long)]
    pub nickname: String,
    #[arg(long, default_value_t = format!("{SERVER_IP}:{SERVER_PORT}"))]
    pub address: String,
}

#[derive(Parser, Resource)]
pub struct ServerCliArgs {
    #[arg(long, default_value_t = 1)]
    pub speed: u32,
    #[arg(long, default_value_t = ("").to_string())]
    pub system: String,
    #[arg(long, default_value_t = format!("{SERVER_IP}:{SERVER_PORT}"))]
    pub address: String,
    #[arg(long)]
    pub zerog: bool,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy, Serialize, Deserialize)]
pub enum GameState {
    Running, // full networked game play
    Waiting, // waiting for clients
    Stopped, // initial state
}

pub fn init_masses<'a>(
    inhabited_mass_id: u64,
    init_data: InitData,
    commands: &'a mut Commands,
    meshes: &'a mut ResMut<Assets<Mesh>>,
    materials: &'a mut ResMut<Assets<StandardMaterial>>,
) {
    for (
        &mass_id,
        &MassInitData {
            inhabitable,
            motion: MassMotion { position, velocity },
            color,
            mass,
        },
    ) in init_data.masses.iter()
    {
        let scale = Vec3::splat(mass_to_radius(mass));
        let mut transform = Transform::from_translation(position).with_scale(scale);
        if inhabitable {
            transform.look_at(Vec3::ZERO, Vec3::Y);
            transform.scale += Vec3::splat(2.5);
        }
        // NOTE: We use unit radius always. We scale as needed.
        let radius = 1.0;
        let mut mass_commands = commands.spawn(physics::PointMassBundle {
            pbr: PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Icosphere {
                    radius,
                    ..Default::default()
                })),
                material: materials.add(color.into()),
                transform,
                ..Default::default()
            },
            momentum: components::Momentum { velocity },
            collider: Collider::ball(radius),
            ..Default::default()
        });
        mass_commands.insert(components::MassID(mass_id));
        if mass_id == inhabited_mass_id {
            mass_commands.insert(components::ClientInhabited);
            mass_commands.remove::<RigidBody>();
            mass_commands.with_children(|child| {
                child.spawn(Camera3dBundle::default());
                child
                    .spawn(PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Icosphere {
                            radius: 0.0005,
                            ..Default::default()
                        })),
                        material: materials.add(Color::WHITE.into()),
                        transform: Transform::from_xyz(0.0, 0.0, -0.2),
                        visibility: Visibility::INVISIBLE,
                        ..Default::default()
                    })
                    .insert(components::Sights);
                child
                    .spawn(PointLightBundle {
                        transform: Transform::from_xyz(0.0, 0.0, -0.15),
                        visibility: Visibility::INVISIBLE,
                        point_light: PointLight {
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(components::Sights);
            });
        } else if inhabitable {
            mass_commands
                .insert(components::Inhabitable)
                .with_children(|child| {
                    // barrel
                    child.spawn(PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Capsule {
                            radius: 0.05,
                            depth: 1.0,
                            ..Default::default()
                        })),
                        material: materials.add(Color::WHITE.into()),
                        transform: Transform::from_rotation(Quat::from_rotation_x(TAU / 4.0))
                            .with_translation(Vec3::Z * -1.5),
                        ..Default::default()
                    });
                    // horizontal stabilizer
                    child.spawn(PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Box::new(2.0, 0.075, 1.0))),
                        material: materials.add(Color::WHITE.into()),
                        transform: Transform::from_translation(Vec3::Z * 0.5),
                        ..Default::default()
                    });
                    // vertical stabilizer
                    child.spawn(PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Box::new(2.0, 0.075, 1.0))),
                        material: materials.add(Color::WHITE.into()),
                        transform: Transform::from_rotation(Quat::from_rotation_z(TAU / 4.0))
                            .with_translation(Vec3::Z * 0.5),
                        ..Default::default()
                    });
                });
        }
    }
}

#[derive(Serialize, Deserialize, Resource, Debug, Copy, Clone)]
pub struct PhysicsConfig {
    pub speed: u32,
    pub zerog: bool,
}

impl Default for PhysicsConfig {
    fn default() -> Self {
        Self {
            speed: 1,
            zerog: false,
        }
    }
}

#[derive(Default, Serialize, Deserialize, Clone, Copy, Debug)]
pub struct MassMotion {
    pub position: Vec3,
    pub velocity: Vec3,
}

#[derive(Default, Serialize, Deserialize, Resource, Debug, Copy, Clone)]
pub struct MassInitData {
    pub inhabitable: bool,
    pub motion: MassMotion,
    pub color: Color,
    pub mass: f32,
}

#[derive(Default, Serialize, Deserialize, Resource, Debug, Clone)]
pub struct InitData {
    pub masses: HashMap<u64, MassInitData>,
}

#[derive(Default, Serialize, Deserialize, Resource, Debug, Clone)]
pub struct GameConfig {
    pub client_mass_map: HashMap<u64, u64>,
    pub physics_config: PhysicsConfig,
    pub init_data: InitData,
}

// Used by the server.
// This is first populated with inhabitable mass IDs given in the InitData
// When a mass is assigned to a client, it is removed from here, therefore
// this same data can and is used to tell when all slots are filled by
// testing for emptyness.
#[derive(Resource, Default)]
pub struct GameStartupData {
    pub unassigned_mass_ids: Vec<u64>,
}
