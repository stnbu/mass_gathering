use bevy::prelude::*;

use bevy_rapier3d::prelude::{Collider, QueryFilter, RapierContext};

use std::collections::HashSet;

mod controls;
pub use controls::*;

mod setup;
pub use setup::*;

use crate::Momentum;

#[derive(Component, PartialEq, Eq)]
pub(crate) enum SpacecraftAR {
    CrosshairsHot,
    CrosshairsCold,
}

#[derive(Debug, Default, Component)]
pub struct Spacecraft;

#[derive(Resource)]
pub struct SpacecraftConfig {
    pub show_impact_explosions: bool,
    pub start_transform: Transform,
    pub impact_magnitude: f32,
    pub radius: f32,
}

impl Default for SpacecraftConfig {
    fn default() -> Self {
        Self {
            show_impact_explosions: true,
            start_transform: Transform::from_xyz(0.0, 0.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y), // Default::default(),
            impact_magnitude: 25.0,
            radius: 1.0,
        }
    }
}

#[derive(Default)]
pub struct Despawned(HashSet<Entity>);

pub(crate) fn set_ar_default_visibility(mut ar_query: Query<(&mut Visibility, &SpacecraftAR)>) {
    for (mut visibility, mode) in ar_query.iter_mut() {
        match mode {
            SpacecraftAR::CrosshairsCold => visibility.is_visible = true,
            SpacecraftAR::CrosshairsHot => visibility.is_visible = false,
        }
    }
}

pub(crate) fn handle_hot_planet(
    spacecraft_query: Query<&Children, With<Spacecraft>>,
    mut ar_query: Query<(&mut Visibility, &SpacecraftAR), Without<Spacecraft>>,
    hot_planet_events: EventReader<HotPlanetEvent>,
) {
    for children in spacecraft_query.iter() {
        if !hot_planet_events.is_empty() {
            for child_id in children.iter() {
                if let Ok((mut visibility, ar_element)) = ar_query.get_mut(*child_id) {
                    match *ar_element {
                        SpacecraftAR::CrosshairsHot => {
                            visibility.is_visible = true;
                        }
                        SpacecraftAR::CrosshairsCold => {
                            visibility.is_visible = false;
                        }
                    }
                }
            }
        }
    }
}

pub struct HotPlanetEvent {
    pub planet: Entity,
    // This is: the direction to the impact site relative to the planet's transform
    pub local_direction: Vec3,
}

pub fn signal_hot_planet(
    planet_query: Query<&Transform, With<Momentum>>,
    spacecraft_query: Query<(Entity, &Transform), With<Spacecraft>>,
    rapier_context: Res<RapierContext>,
    mut hot_planet_events: EventWriter<HotPlanetEvent>,
) {
    for (spacecraft_id, pov) in spacecraft_query.iter() {
        // TODO: can we use "native" raycasting here?
        // https://docs.rs/bevy/0.9.0/bevy/render/camera/struct.Camera.html#method.viewport_to_world
        let ray_origin = pov.translation;
        let ray_direction = -1.0 * pov.local_z();
        let intersection = rapier_context.cast_ray(
            ray_origin,
            ray_direction,
            150.0, // what's reasonable here...?
            true,
            QueryFilter::only_dynamic().exclude_collider(spacecraft_id),
        );

        if let Some((planet, distance)) = intersection {
            if let Ok(planet_transform) = planet_query.get(planet) {
                let global_impact_site = ray_origin + (ray_direction * distance);
                let local_direction =
                    (global_impact_site - planet_transform.translation).normalize();
                let event = HotPlanetEvent {
                    planet,
                    local_direction,
                };
                hot_planet_events.send(event);
            }
        }
    }
}
