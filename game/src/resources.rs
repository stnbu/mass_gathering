use crate::*;
use bevy_rapier3d::prelude::{Collider, RigidBody};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy, Serialize, Deserialize)]
pub enum GameState {
    Running, // full networked game play
    Waiting, // waiting for clients
    Stopped, // initial state
}

pub fn init_masses(inhabited_mass_id: u64, init_data: InitData, commands: &mut Commands) {
    for (
        &mass_id,
        &MassInitData {
            inhabitable,
            motion: MassMotion { position, velocity },
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
            transform_bundle: TransformBundle::from_transform(transform),
            momentum: components::Momentum { velocity },
            // collider only has a RADIUS
            collider: Collider::ball(radius),
            ..Default::default()
        });
        mass_commands.insert(components::MassID(mass_id));
        if inhabitable {
            mass_commands.remove::<RigidBody>();
            if mass_id == inhabited_mass_id {
                mass_commands.insert(components::ClientInhabited);
            } else {
                mass_commands.insert(components::Inhabitable);
            }
        }
        debug!(
            "Spawned inhabitable={inhabitable} mass {mass_id} at {:?} ({:?})",
            transform.translation,
            mass_commands.id()
        );
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

impl GameConfig {
    pub fn is_capacity(&self) -> bool {
        let assigned_count = self.client_mass_map.len();
        let inhabitable_count = self
            .init_data
            .masses
            .iter()
            .filter(|(_, mass)| mass.inhabitable)
            // FIXME: shouldn't there just be a single method somewhere?
            .collect::<Vec<_>>()
            .len();
        inhabitable_count == assigned_count
    }

    pub fn get_assigned_mass_id(&mut self, client_id: u64) -> Result<u64, &str> {
        let inhabited_mass_ids = self
            .client_mass_map
            .iter()
            .map(|(_, &mass_id)| mass_id)
            .collect::<HashSet<u64>>();
        for (&inhabitable_mass_id, _) in self
            .init_data
            .masses
            .iter()
            .filter(|(_, mass)| mass.inhabitable)
        {
            if !inhabited_mass_ids.contains(&inhabitable_mass_id) {
                if let Some(mass_id) = self.client_mass_map.insert(client_id, inhabitable_mass_id) {
                    panic!("Client {client_id} already assigned {mass_id}")
                }
                return Ok(inhabitable_mass_id);
            }
        }
        Err("No free mass IDs")
    }
}
