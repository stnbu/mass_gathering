use crate::*;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy, Serialize, Deserialize)]
pub enum GameState {
    Running, // full networked game play
    Waiting, // waiting for clients
    Stopped, // initial state
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
    pub color: [f32; 4],
    pub mass: f32,
}

impl From<MassInitData> for Transform {
    fn from(mass_init_data: MassInitData) -> Transform {
        let position = mass_init_data.motion.position;
        let scale = mass_to_scale(mass_init_data.mass);
        let mut transform = Transform::from_translation(position).with_scale(scale);
        if mass_init_data.inhabitable {
            transform.look_at(Vec3::ZERO, Vec3::Y);
            transform.scale += Vec3::splat(2.5);
        }
        transform
    }
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

// FIXME: This implimentation is just terrible.
// And it seems odd that a "config" has a concept of
// "pop id" and whatnot. Lotta wat.
impl GameConfig {
    pub fn get_unassigned_mass_ids(&self) -> HashSet<u64> {
        let assigned_mass_ids = self
            .client_mass_map
            .iter()
            .map(|(_, v)| *v)
            .collect::<HashSet<u64>>();
        let inhabitable_mass_ids = self
            .init_data
            .masses
            .iter()
            .filter(|(_, mass)| mass.inhabitable)
            .map(|(n, _)| *n)
            .collect::<HashSet<u64>>();
        inhabitable_mass_ids
            .difference(&assigned_mass_ids)
            .map(|n| *n)
            .collect::<HashSet<u64>>()
    }

    pub fn is_capacity(&self) -> bool {
        let is_capacity = self.get_unassigned_mass_ids().is_empty();
        is_capacity
    }

    pub fn get_free_id(&self) -> Result<u64, &str> {
        self.get_unassigned_mass_ids()
            .iter()
            .next()
            .copied()
            .ok_or("No more free IDs!")
    }
}
