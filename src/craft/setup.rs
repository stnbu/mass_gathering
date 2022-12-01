use bevy::log::debug;
use bevy::prelude::{
    shape, Assets, BuildChildren, Camera3dBundle, Color, Commands, Mesh, PbrBundle, Res, ResMut,
    StandardMaterial, Transform, Vec3, Visibility,
};

use super::*;

pub fn spacecraft_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    config: Res<SpacecraftConfig>,
) {
    let radius = 1.0;
    let spacecraft = commands
        .spawn(TransformBundle::from_transform(
            Transform::from_xyz(10.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ))
        .insert(VisibilityBundle::default())
        .insert(Spacecraft)
        .insert(Momentum {
            velocity: Vec3::ZERO,
            mass: mass_to_radius(radius),
            ..Default::default()
        })
        //.insert(Collider::ball(radius))
        .with_children(|child| {
            child.spawn(Camera3dBundle {
                transform: Transform::from_xyz(0.0, 0.0, 0.0).looking_at(-Vec3::Z, Vec3::Y),
                ..Default::default()
            });
            // Possibly the worst way to implement "crosshairs" evar.
            //
            // This coefficient to make the crosshairs "as close as possible" to our "eyeball"
            // (By complete luck, this gives us a nice flickering hair...)
            let distance = 0.025;
            child
                .spawn(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Icosphere {
                        radius: 0.01 * distance,
                        ..Default::default()
                    })),
                    material: materials.add(Color::LIME_GREEN.into()),
                    transform: Transform::from_xyz(0.0, 0.0, -7.0 * distance),
                    visibility: Visibility { is_visible: false },
                    ..Default::default()
                })
                .insert(SpacecraftAR::CrosshairsCold);
            child
                .spawn(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Box::new(
                        0.005 * distance,
                        5.0 * distance,
                        0.08 * distance,
                    ))),
                    material: materials.add(Color::GREEN.into()),
                    transform: Transform::from_xyz(0.0, 0.0, -7.0 * distance),
                    visibility: Visibility { is_visible: false },
                    ..Default::default()
                })
                .insert(SpacecraftAR::CrosshairsHot);
            child
                .spawn(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Box::new(
                        5.0 * distance,
                        0.005 * distance,
                        0.08 * distance,
                    ))),
                    material: materials.add(Color::GREEN.into()),
                    transform: Transform::from_xyz(0.0, 0.0, -6.0 * distance),
                    visibility: Visibility { is_visible: false },
                    ..Default::default()
                })
                .insert(SpacecraftAR::CrosshairsHot);
        })
        .id();
    debug!("Spawned spacecraft with entity {spacecraft:?}");
}
