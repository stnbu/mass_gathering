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

        let xx = inhabitable_mass_ids
            .difference(&assigned_mass_ids)
            .map(|n| *n)
            .collect::<HashSet<u64>>();
        //config: {self:#?}
        warn!(
            "

assigned: {assigned_mass_ids:?}
inhabitable: {inhabitable_mass_ids:?}

inhabitable IDs: {xx:?}

"
        );
        xx
    }

    pub fn is_capacity(&self) -> bool {
        let is_capacity = self.get_unassigned_mass_ids().is_empty();
        warn!("is capacity? {is_capacity}");
        is_capacity
    }

    pub fn get_free_id(&self) -> Result<u64, &str> {
        warn!("booo");
        self.get_unassigned_mass_ids()
            .iter()
            .next()
            .copied()
            .ok_or("No more free IDs!")
    }
}
