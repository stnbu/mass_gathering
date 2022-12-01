use bevy::log::debug;
use bevy::prelude::{
    shape, Assets, BuildChildren, Camera3dBundle, Color, Commands, Mesh, PbrBundle, Res, ResMut,
    StandardMaterial, Transform, Vec3, Visibility,
};

use crate::PointMassBundle;

use super::*;

pub fn spacecraft_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    config: Res<SpacecraftConfig>,
) {
    let spacecraft = commands
        /*
            .spawn(PointMassBundle {
                pbr: PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Icosphere {
                        radius: 0.1,
                        ..Default::default()
                    })),
                    material: materials.add(Color::rgba(1.0, 1.0, 1.0, 0.5).into()),
                    visibility: Visibility { is_visible: true },
                    ..Default::default()
                },
                momentum: Momentum {
                    velocity: Vec3::ZERO,
                    mass: mass_to_radius(10.0),
                    ..Defappult::default()
                },
                collider: Collider::ball(1.0),
                ..Default::default()
            })
        */
        .spawn(TransformBundle::from_transform(config.start_transform))
        .insert(VisibilityBundle::default())
        .insert(Spacecraft {
            speed: config.start_speed,
        })
        .insert(Momentum {
            velocity: Vec3::ZERO,
            mass: mass_to_radius(10.0),
            ..Default::default()
        })
        .with_children(|child| {
            child.spawn(Camera3dBundle {
                transform: Transform::from_xyz(0.0, 0.0, 0.0).looking_at(-Vec3::Z, Vec3::Y),
                ..Default::default()
            });
            // Possibly the worst way to implement "crosshairs" evar.
            child
                .spawn(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Icosphere {
                        radius: 0.01,
                        ..Default::default()
                    })),
                    material: materials.add(Color::LIME_GREEN.into()),
                    transform: Transform::from_xyz(0.0, 0.0, -7.0),
                    visibility: Visibility { is_visible: false },
                    ..Default::default()
                })
                .insert(SpacecraftAR::CrosshairsCold);
            child
                .spawn(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Box::new(0.005, 5.0, 0.08))),
                    material: materials.add(Color::GREEN.into()),
                    transform: Transform::from_xyz(0.0, 0.0, -7.0),
                    visibility: Visibility { is_visible: false },
                    ..Default::default()
                })
                .insert(SpacecraftAR::CrosshairsHot);
            child
                .spawn(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Box::new(5.0, 0.005, 0.08))),
                    material: materials.add(Color::GREEN.into()),
                    transform: Transform::from_xyz(0.0, 0.0, -6.0),
                    visibility: Visibility { is_visible: false },
                    ..Default::default()
                })
                .insert(SpacecraftAR::CrosshairsHot);
        })
        .id();
    debug!("Spawned spacecraft with entity {spacecraft:?}");
}
