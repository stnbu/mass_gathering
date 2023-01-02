/// Hello there. I'm just getting started. These are not used; I'm knocking around ideas.
use bevy::prelude::{App, EventReader, Plugin, SystemSet, Vec3};

pub trait NetworkableSystemSet {
    fn is_networked_game() -> bool;
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
enum NetworkState {
    Waiting,
    Connected,
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
enum GameType {
    Standalone,
    Networked(NetworkState),
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameType::Networked(NetworkState::Connected)), // build
        );
    }
}

/*
       app.add_system_set(
           SystemSet::on_update(GameState::Running)
               .with_system(inhabitant::control)
               .with_system(inhabitant::rotate_client_inhabited_mass),
       );
*/

// * Comes from user input
// * I am updated immedately via bevy events.
// * Others are updated based upon broadcasts.
// * "Cosmetic", meaning that this does not affect physics or game timeline.
pub struct ClientRotate {}

// * Used only "locally", never sent to server
pub struct HotPlanet {
    pub mass_id: u64,
    pub local_direction: Vec3,
}

pub struct DespawnMass {
    pub mass_id: u64,
}

pub struct MassCollision {
    pub major_mass_id: u64,
    pub minor_mass_id: u64,
}

pub enum MassDelta {
    Velocity(Vec3),
    Postion(Vec3),
}

// * Broadcast to all clients
// * Used to animate projectile
// * Not handed dirctly by client, wait for re-broadcast
// * Used by server to track projectile
pub struct FireProjectile {
    pub start: u64, // time since start of game... nanoseconds?
    pub inhabitant_id: u64,
    pub target_mass_id: u64,
    pub target_local_direction: Vec3,
}

use crate::MassMotion;
use std::collections::{HashMap, HashSet};

// * Not sent to server
// * Eminates from server for all collisions
pub struct ProjectileCollision {
    pub mass_id: u64,
    pub local_direction: Vec3,
}

pub struct UniverseReset {
    pub id: u64,
    pub deadline: u64,
    pub physics: HashMap<u64, MassMotion>,
    pub unacknowledged_clients: HashSet<u64>,
}

pub struct OpenUniverseResets {
    pub resets: HashMap<u64, UniverseReset>,
}

use bevy::prelude::error;

impl OpenUniverseResets {
    pub fn acknowledge(&mut self, client_id: u64, reset_id: u64) {
        if let Some(reset) = self.resets.get_mut(&reset_id) {
            if !reset.unacknowledged_clients.remove(&client_id) {
                error!(
                    "Client {client_id} not among unacknowledged: {:?}; re-acknowledgement?",
                    reset.unacknowledged_clients
                );
            }
            if reset.unacknowledged_clients.is_empty() {
                let msg = format!(
                    "Tried to remove reset id {reset_id} from {:?} (non-existant key)",
                    self.resets.keys()
                );
                self.resets.remove(&reset_id).expect(&msg);
            }
        } else {
            panic!("{reset_id}: unknown reset id");
        }
    }
    pub fn abort_on_overdue(&self) {
        for (_, reset) in self.resets.iter() {
            let now = 0_u64; // FIXME FIXME
            let grace: u64 = 1_000_000_000;
            if now - reset.deadline > grace {
                panic!("Reset {} was more than {}ns overdue!", reset.id, grace);
            }
        }
    }
}

// use bevy::ecs::schedule::ShouldRun;
// pub fn pending_universe_resets(resets: Option<EventReader<UniverseReset>>) -> ShouldRun {
//     match resets {
//         Some(reset) if !reset.is_empty() => ShouldRun::Yes,
//         _ => ShouldRun::No,
//     }
// }
