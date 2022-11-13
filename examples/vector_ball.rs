use bevy::prelude::*;
use mass_gathering::prelude::*;

fn main() {
    App::new()
        .add_plugins(FullGame)
        .insert_resource(SpacecraftConfig {
            start_transform: Transform::from_xyz(0.0, 0.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .add_startup_system(setup)
        .run();
}

const BALL_RADIUS: f32 = 3.5;
const FLOAT_HEIGHT: f32 = 2.0;

const VECTOR_LENGTH: f32 = 14.0;
const VECTOR_SCALE: f32 = 1.0;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let vector_cylinder_length = VECTOR_LENGTH - BALL_RADIUS - FLOAT_HEIGHT - 2.0 * VECTOR_SCALE;

    let momentum_vector = commands
        .spawn_bundle(SpatialBundle::default())
        .with_children(|child| {
            child.spawn_bundle(PbrBundle {
                mesh: meshes.add(
                    (Cone {
                        radius: 2.0 * VECTOR_SCALE,
                        height: 2.0 * VECTOR_SCALE,
                        ..Default::default()
                    })
                    .into(),
                ),
                transform: Transform::from_xyz(0.0, VECTOR_LENGTH - 2.0 * VECTOR_SCALE, 0.0),
                material: materials.add(Color::GREEN.into()),
                ..Default::default()
            });
            child.spawn_bundle(PbrBundle {
                mesh: meshes.add(
                    (Cylinder {
                        height: vector_cylinder_length,
                        radius_bottom: VECTOR_SCALE,
                        radius_top: VECTOR_SCALE,
                        ..Default::default()
                    })
                    .into(),
                ),
                transform: Transform::from_xyz(
                    0.0,
                    vector_cylinder_length * 0.5 + BALL_RADIUS + FLOAT_HEIGHT,
                    0.0,
                ),
                material: materials.add(Color::GREEN.into()),
                ..Default::default()
            });
        })
        .id();
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(
                (shape::Icosphere {
                    radius: BALL_RADIUS,
                    ..Default::default()
                })
                .into(),
            ),
            material: materials.add(Color::GREEN.into()),
            ..Default::default()
        })
        .add_child(momentum_vector);
}
