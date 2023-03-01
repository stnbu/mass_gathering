/// Resources regardless of package (sort of).
/// Things that are both `Component` and `Resource` also
/// go here (I think).
use crate::*;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy, Serialize, Deserialize)]
/// refactor_tags: game_state
pub enum GameState {
    Running, // full networked game play
    Waiting, // waiting for clients
    Stopped, // initial state
}

impl std::fmt::Display for GameState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let display = match self {
            Self::Running => "running",
            Self::Waiting => "waiting",
            Self::Stopped => "stopped",
        };
        write!(f, "{}", display)
    }
}

#[derive(Serialize, Deserialize, Resource, Debug, Copy, Clone)]
/// NOTE: Isn't "physics" part of "simulation"? Could these values be merged into something? If there is a
/// "simulation plugin" then this should probably be handled by that.
///
/// refactor_tags: simulation, refactor
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
/// NOTE: This also wants to be combined with something, somehow. This is only used by GameConfig/InitData
///
/// refactor_tags: refactor, simulation
pub struct MassMotion {
    pub position: Vec3,
    pub velocity: Vec3,
}

#[derive(Serialize, Deserialize, Resource, Debug, Copy, Clone)]
/// refactor_tags: simulation
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
/// refactor_tags: simulation
pub struct InitData {
    pub masses: HashMap<u64, MassInitData>,
}

#[derive(Default, Serialize, Deserialize, Resource, Debug, Clone)]
/// NOTE: This tagged "refactor" mostly because of its weid impl.
///
/// refactor_tags: simulation, to_client, refactor
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
            .filter(|(_, mass)| mass.inhabitation.vacant())
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

// Both a `Resource`, because it is used in `UiState`, and also
// a `Component` because it is used to mark camera entities.
#[derive(Resource, Debug, Component, PartialEq, Clone)]
/// refactor_tags: gui, cameras, testing
pub enum CameraTag {
    Client,
    Objective,
}

// Converting to an `isize` let's us re-use the component as `Camera::priority`
impl From<&CameraTag> for isize {
    fn from(active_camera: &CameraTag) -> Self {
        match active_camera {
            CameraTag::Client => 0,
            CameraTag::Objective => 1,
        }
    }
}

impl std::fmt::Display for CameraTag {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let display = match self {
            Self::Client => "client",
            Self::Objective => "objective",
        };
        write!(f, "{}", display)
    }
}

#[derive(Resource, Debug)]
/// refactor_tags: gui, user_input, cameras, egui
pub struct UiState {
    pub camera: CameraTag,
    pub show_info: bool,
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            camera: CameraTag::Client,
            show_info: true,
        }
    }
}
