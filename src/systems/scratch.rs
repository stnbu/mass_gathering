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
