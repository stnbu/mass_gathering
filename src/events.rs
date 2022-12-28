/// Hello there. I'm just getting started. These are not used; I'm knocking around ideas.
use bevy::prelude::Vec3;

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
use std::collections::HashMap;

pub struct NewUniverse {
    pub deadline: u64,
    pub physics: HashMap<u64, MassMotion>,
}

// * Not sent to server
// * Eminates from server for all collisions
pub struct ProjectileCollision {
    pub mass_id: u64,
    pub local_direction: Vec3,
}
