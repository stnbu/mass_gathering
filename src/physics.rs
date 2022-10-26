use bevy::prelude::*;
use std::f32::consts::PI;

fn radius_to_mass(radius: f32) -> f32 {
    (4.0 / 3.0) * PI * radius.powf(3.0)
}

fn mass_to_radius(mass: f32) -> f32 {
    ((mass * (3.0 / 4.0)) / PI).powf(1.0 / 3.0)
}

pub fn spawn_planet<'a>(
    radius: f32,
    position: Vec3,
    velocity: Vec3,
    color: Color,
    commands: &'a mut Commands,
    meshes: &'a mut ResMut<Assets<Mesh>>,
    materials: &'a mut ResMut<Assets<StandardMaterial>>,
) {
    let mass = radius_to_mass(radius);
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Icosphere {
            radius,
            ..Default::default()
        })),
        material: materials.add(color.into()),
        transform: Transform::from_translation(position),
        ..Default::default()
    });
}
