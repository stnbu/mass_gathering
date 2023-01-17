use crate::*;

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
