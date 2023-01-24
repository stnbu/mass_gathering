use crate::*;
use bevy_rapier3d::prelude::Collider;
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

#[derive(Serialize, Deserialize, Component, Debug, Copy, Clone)]
pub struct ClientData {
    pub inhabited_mass_id: u64,
}

#[derive(Default, Resource, Debug)]
pub struct Lobby {
    pub clients: HashMap<u64, ClientData>,
}

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

#[derive(Resource, Default, Clone)]
pub struct MassIDToEntityMap(pub HashMap<u64, Entity>);

impl MassIDToEntityMap {
    pub fn get_entities<const N: usize>(&self, mass_ids: [u64; N]) -> Result<[Entity; N], &str> {
        let mut entities: Vec<Entity> = Vec::new();
        for id in mass_ids.iter() {
            if let Some(value) = self.0.get(id) {
                entities.push(*value);
            } else {
                return Result::Err("Failed to find mass id {id}");
            }
        }
        Result::Ok(entities.try_into().unwrap()) // omg becky
    }
}

#[derive(Default, Serialize, Deserialize, Clone, Copy, Debug)]
pub struct MassMotion {
    pub position: Vec3,
    pub velocity: Vec3,
}

#[derive(Default, Serialize, Deserialize, Clone, Copy, Debug)]
pub struct MassInitData {
    pub inhabitable: bool,
    pub motion: MassMotion,
    pub color: Color,
    pub mass: f32, // WIP: no radius!
}

#[derive(Default, Serialize, Deserialize, Resource, Debug)]
pub struct InitData {
    pub masses: HashMap<u64, MassInitData>,
}

impl Clone for InitData {
    fn clone(&self) -> Self {
        let mut masses = HashMap::new();
        masses.extend(&self.masses);
        Self { masses }
    }

    fn clone_from(&mut self, source: &Self) {
        let mut masses = HashMap::new();
        masses.extend(&source.masses);
        self.masses = masses;
    }
}

pub fn init_masses<'a>(
    init_data: InitData,
    commands: &'a mut Commands,
    meshes: &'a mut ResMut<Assets<Mesh>>,
    materials: &'a mut ResMut<Assets<StandardMaterial>>,
) -> MassIDToEntityMap {
    let mut mass_to_entity_map = MassIDToEntityMap::default();
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
        if inhabitable {
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
        mass_to_entity_map.0.insert(mass_id, mass_commands.id());
    }
    mass_to_entity_map
}
