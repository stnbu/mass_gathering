use crate::*;
use bevy_rapier3d::prelude::{QueryFilter, RapierContext};

pub fn pimples() -> resources::InitData {
    let mut init_data = resources::InitData::default();
    let position = Vec3::X * 10.0;
    let velocity = Vec3::Y * 0.035;
    let radius = 1.0;
    init_data.masses.insert(
        0,
        resources::MassInitData {
            inhabitable: false,
            motion: resources::MassMotion {
                position: position * 1.0,
                velocity: velocity * -1.0,
            },
            color: Color::RED,
            radius,
        },
    );
    init_data.masses.insert(
        1,
        resources::MassInitData {
            inhabitable: true,
            motion: resources::MassMotion {
                position: position * -1.0,
                velocity: velocity * 1.0,
            },
            color: Color::BLUE,
            radius,
        },
    );
    init_data
}

pub fn pimples_xz_translate(
    mut transform_query: Query<&mut Transform, With<components::ClientInhabited>>,
    keys: Res<Input<KeyCode>>,
) {
    let nudge = 0.05;
    let mut x = 0.0;
    let mut z = 0.0;

    for key in keys.get_pressed() {
        match key {
            KeyCode::Up => {
                x += nudge;
            }
            KeyCode::Down => {
                x -= nudge;
            }
            KeyCode::Left => {
                z -= nudge;
            }
            KeyCode::Right => {
                z += nudge;
            }
            _ => (),
        }
    }
    if let Ok(mut transform) = transform_query.get_single_mut() {
        transform.translation += Vec3::new(x, 0.0, z);
    }
}

pub fn pimples_rotate_target(
    mut target_query: Query<&mut Transform, Without<components::ClientInhabited>>,
    inhabited_mass_query: Query<&Transform, With<components::ClientInhabited>>,
    rapier_context: Res<RapierContext>,
    keys: Res<Input<KeyCode>>,
) {
    if let Ok(client_pov) = inhabited_mass_query.get_single() {
        let ray_origin = client_pov.translation;
        let ray_direction = -client_pov.local_z();
        let intersection = rapier_context.cast_ray(
            ray_origin,
            ray_direction,
            150.0,
            false,
            QueryFilter::only_dynamic(),
        );
        if let Some((target, _)) = intersection {
            if let Ok(mut target_transform) = target_query.get_mut(target) {
                let nudge = TAU / 10000.0;
                let keys_scaling = 10.0;
                let mut rotation = Vec3::ZERO;
                if keys.pressed(KeyCode::RShift) {
                    for key in keys.get_pressed() {
                        match key {
                            // pitch
                            KeyCode::W => {
                                rotation.x += nudge;
                            }
                            KeyCode::S => {
                                rotation.x -= nudge;
                            }
                            // yaw
                            KeyCode::A => {
                                rotation.y += nudge;
                            }
                            KeyCode::D => {
                                rotation.y -= nudge;
                            }
                            // roll
                            KeyCode::Z => {
                                rotation.z -= nudge;
                            }
                            KeyCode::X => {
                                rotation.z += nudge;
                            }
                            _ => (),
                        }
                    }
                }
                if rotation.length() > 0.0000001 {
                    let frame_time = 1.0;
                    rotation *= keys_scaling * frame_time;
                    target_transform.rotate(Quat::from_axis_angle(Vec3::X, rotation.x));
                    target_transform.rotate(Quat::from_axis_angle(Vec3::Z, rotation.z));
                    target_transform.rotate(Quat::from_axis_angle(Vec3::Y, rotation.y));
                }
            }
        }
    }
}
