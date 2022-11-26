use bevy::log::debug;
use bevy::prelude::{
    shape, Assets, BuildChildren, Camera3dBundle, Color, Commands, Mesh, PbrBundle, PointLight,
    PointLightBundle, Res, ResMut, StandardMaterial, Transform, TransformBundle, Vec3, Visibility,
    VisibilityBundle,
};
use std::collections::HashMap;

use super::*;
use crate::mg_shapes::*;

pub fn spacecraft_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut vector_ball_data: ResMut<VectorBallData>,
    config: Res<SpacecraftConfig>,
) {
    let spacecraft = commands
        .spawn(TransformBundle::from_transform(config.start_transform))
        .insert(VisibilityBundle::default())
        .insert(Spacecraft {
            speed: config.start_speed,
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

            // Various lights for seeing
            child.spawn(PointLightBundle {
                transform: Transform::from_xyz(10.0, -10.0, -25.0),
                point_light: PointLight {
                    intensity: 5000.0 * 1.7,
                    range: 1000.0,
                    ..Default::default()
                },
                ..Default::default()
            });
            child.spawn(PointLightBundle {
                transform: Transform::from_xyz(-10.0, 5.0, -35.0),
                point_light: PointLight {
                    intensity: 5000.0 * 1.5,
                    range: 1000.0,
                    ..Default::default()
                },
                ..Default::default()
            });
            child.spawn(PointLightBundle {
                transform: Transform::from_xyz(30.0, -20.0, 80.0),
                point_light: PointLight {
                    intensity: 1000000.0 * 0.7,
                    range: 1000.0,
                    ..Default::default()
                },
                ..Default::default()
            });
            child.spawn(PointLightBundle {
                transform: Transform::from_xyz(-30.0, 10.0, 100.0),
                point_light: PointLight {
                    intensity: 1000000.0 * 0.8,
                    range: 1000.0,
                    ..Default::default()
                },
                ..Default::default()
            });
        })
        .id();
    debug!("Spawned spacecraft with entity {spacecraft:?}");

    // Vector Ball Stuff... //

    // Vector part colors: [(element, (cone_color, cylinder_color))]
    let vector_colors = HashMap::from([
        (
            VectorBallElement::Momentum,
            (
                // Cone
                StandardMaterial {
                    base_color: Color::GRAY,
                    ..Default::default()
                },
                // Cylinder
                StandardMaterial {
                    base_color: Color::OLIVE,
                    ..Default::default()
                },
            ),
        ),
        (
            VectorBallElement::Force,
            (
                StandardMaterial {
                    base_color: Color::BLUE,
                    ..Default::default()
                },
                StandardMaterial {
                    base_color: Color::DARK_GREEN,
                    ..Default::default()
                },
            ),
        ),
    ]);

    let ball = commands
        .spawn(PbrBundle {
            visibility: Visibility { is_visible: false },
            mesh: meshes.add(
                (shape::Icosphere {
                    radius: BALL_RADIUS * 0.8,
                    ..Default::default()
                })
                .into(),
            ),
            material: materials.add(Color::rgba(1.0, 1.0, 1.0, 0.7).into()),
            ..Default::default()
        })
        .insert(VectorBallElement::Ball)
        .id();
    vector_ball_data.ball = Some(ball);
    commands.entity(spacecraft).add_child(ball);

    let container = commands
        .spawn(PbrBundle {
            visibility: Visibility { is_visible: false },
            mesh: meshes.add(shape::Box::new(2.0, 2.0, 2.0).into()),
            material: materials.add(Color::rgba(0.0, 1.0, 1.0, 0.01).into()),
            ..Default::default()
        })
        .insert(VectorBallElement::Container)
        .id();
    vector_ball_data.ball = Some(ball);
    commands.entity(spacecraft).add_child(ball);

    vector_ball_data.container = Some(container);
    commands.entity(spacecraft).add_child(container);

    [VectorBallElement::Momentum, VectorBallElement::Force]
        .iter()
        .for_each(|element| {
            let (cone_color, cylinder_color) = vector_colors.get(element).unwrap();
            let cone = commands
                .spawn(PbrBundle {
                    visibility: Visibility { is_visible: false },
                    mesh: meshes.add(
                        (Cone {
                            radius: CONE_RADIUS,
                            height: CONE_HEIGHT,
                            ..Default::default()
                        })
                        .into(),
                    ),
                    material: materials.add(cone_color.clone()),
                    ..Default::default()
                })
                .insert(*element)
                .id();
            let cylinder = commands
                .spawn(PbrBundle {
                    visibility: Visibility { is_visible: false },
                    mesh: meshes.add(
                        (Cylinder {
                            height: 1.0,
                            radius_bottom: CYLINDER_RADIUS,
                            radius_top: CYLINDER_RADIUS,
                            ..Default::default()
                        })
                        .into(),
                    ),
                    material: materials.add(cylinder_color.clone()),
                    ..Default::default()
                })
                .insert(*element)
                .id();

            vector_ball_data
                .vectors
                .insert(*element, VectorParts { cylinder, cone });

            commands.entity(spacecraft).push_children(&[cylinder, cone]);
        });
}
