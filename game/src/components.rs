use crate::*;
use bevy::prelude::Component;

#[derive(Component)]
pub struct MassID(pub u64);

#[derive(Component)]
pub struct ClientInhabited;

#[derive(Component)]
pub struct Inhabitable;

#[derive(Component, Debug, Default)]
pub struct Momentum {
    pub velocity: Vec3,
}

// FIXME: Very client-specific. Relocate? Justification? Paperwork? Bike shaving?
/// Sights as in "gun sights"
#[derive(Component)]
pub struct Sights;
