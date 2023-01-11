use bevy::prelude::Component;

#[derive(Component)]
pub struct MassID(pub u64);

#[derive(Component)]
pub struct ClientInhabited;

#[derive(Component)]
pub struct Inhabitable;
