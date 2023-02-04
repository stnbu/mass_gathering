use crate::*;

pub fn demo_shooting() -> resources::InitData {
    let mut init_data = resources::InitData::default();
    let id_base = 0;

    let velocity = Vec3::ZERO;
    let x = 10.0;
    let y = 0.0;
    for i in 0..5 {
        let i = i * 4;
        let mass_id = id_base + i;
        let z = (i as f32 - 10.0) * 2.5;
        let position = Vec3::new(x, y, z);
        let motion = resources::MassMotion { position, velocity };
        let inhabitable = false;
        let mass = radius_to_mass(i as f32 / 5.0 + 1.0);
        let color = [1.0, 0.0, 0.0, 1.0];
        init_data.masses.insert(
            mass_id,
            resources::MassInitData {
                inhabitable,
                motion,
                mass,
                color,
            },
        );
    }
    let mass = radius_to_mass(3.0);
    let color = [1.0, 0.0, 0.0, 1.0];
    init_data.masses.insert(
        id_base + 21,
        resources::MassInitData {
            inhabitable: true,
            motion: resources::MassMotion {
                position: Vec3::new(-10.0, 0.0, 0.0),
                velocity: Vec3::ZERO,
            },
            mass,
            color,
        },
    );
    init_data
}

pub fn get_system(name: &str) -> impl (Fn() -> resources::InitData) {
    match name {
        "demo_shooting" => demo_shooting,
        _ => panic!("No such system: {name}"),
    }
}
