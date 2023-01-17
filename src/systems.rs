use crate::*;
use rand::Rng;

/// Old rando from way back
pub fn old_rando() -> resources::InitData {
    // FIXME: This a baad one: If we have a starting position or velocity of
    // 0,0,0 we end up with NaN,NaN,NaN after a few frames. WHY?? -- we do not
    // know. Tack this on to be sure :-/
    let tiny_vec_d = Vec3::ONE * 0.000001;

    let mut init_data = resources::InitData::default();

    let mut rng = rand::thread_rng();
    let mut rf = || rng.gen::<f32>();
    let pair_count = 18;
    let mut mass_id = 2000;
    for _ in 0..pair_count {
        let position = latlon_to_cartesian(rf(), rf()) * (rf() * 40.0 + 10.0);
        let velocity = latlon_to_cartesian(rf(), rf()) * Vec3::new(10.0, rf() * 0.1, 10.0) * 0.1;
        let radius = rf() + 2.0;
        for side in [-1.0, 1.0] {
            let color = Color::rgb(rf(), rf(), rf());
            let position = position * side + tiny_vec_d;
            let velocity = velocity * side + tiny_vec_d;
            let mass_init_data = resources::MassInitData {
                inhabitable: false,
                motion: resources::MassMotion { position, velocity },
                color,
                radius,
            };
            init_data.masses.insert(mass_id, mass_init_data);
            mass_id += 1;
        }
    }
    let inhabitable_distance = 70.0;
    for (x, y, z) in [(1, 0, 0), (0, 1, 0), (0, 0, 1)] {
        let velocity = Vec3::ZERO + tiny_vec_d;
        let color_tweak = match (x, y, z) {
            (1, 0, 0) => 1.0,
            (0, 1, 0) => 2.0,
            (0, 0, 1) => 3.0,
            _ => panic!("no!"),
        };
        let position = Vec3::new(x as f32, y as f32, z as f32) * inhabitable_distance + tiny_vec_d;
        let color = Color::rgb(17.0, 19.0 / color_tweak, 23.0 * color_tweak);
        let radius = 1.0;
        let mass_init_data = resources::MassInitData {
            inhabitable: true,
            motion: resources::MassMotion { position, velocity },
            color,
            radius,
        };
        init_data.masses.insert(mass_id, mass_init_data);
        mass_id += 1;
    }
    init_data
}

/// Make some interesting masses
pub fn cubic() -> resources::InitData {
    let mut init_data = resources::InitData::default();

    let mut mass_id = 2000;
    let radius = 0.5;
    let from_origin = 9.0;
    for n in [(1, 0, 0), (0, 1, 0), (0, 0, 1)] {
        for side in [1.0, -1.0] {
            let fun_factor = 1.0 + (mass_id as f32 - 2000.0) / 20.0;

            let (a, b, c) = n;
            let speed = 0.15;
            let position = Vec3::new(
                a as f32 * side * from_origin,
                b as f32 * side * from_origin,
                c as f32 * side * from_origin,
            );
            let velocity = match (a, b, c) {
                (1, 0, 0) => Vec3::Y * side,
                (0, 1, 0) => Vec3::Z * side,
                (0, 0, 1) => Vec3::X * side,
                _ => panic!(),
            } * speed;
            let (r, g, b) = (a as f32, b as f32, c as f32);
            let plus_side = side > 0.0;
            let color = if plus_side {
                Color::rgba(r, g, b, 0.8)
            } else {
                Color::rgba((1.0 - r) / 2.0, (1.0 - g) / 2.0, (1.0 - b) / 2.0, 0.8)
            };
            let velocity = if c == 1 {
                velocity
            } else {
                velocity * fun_factor
            };
            let radius = if a == 1 { radius } else { radius * fun_factor };

            let position = if c == 1 {
                position
            } else {
                position * fun_factor
            };

            let mass_init_data = resources::MassInitData {
                inhabitable: false,
                motion: resources::MassMotion { position, velocity },
                color,
                radius,
            };
            init_data.masses.insert(mass_id, mass_init_data);
            mass_id += 1;
        }
    }

    let inhabitable_distance = 20.0;
    for (x, y, z) in [(1, 0, 0), (0, 1, 0), (0, 0, 1)] {
        let velocity = Vec3::ZERO;
        let color_tweak = match (x, y, z) {
            (1, 0, 0) => 1.0,
            (0, 1, 0) => 2.0,
            (0, 0, 1) => 3.0,
            _ => panic!("no!"),
        };
        let position = Vec3::new(x as f32, y as f32, z as f32) * inhabitable_distance;
        let color = Color::rgb(17.0, 19.0 / color_tweak, 23.0 * color_tweak);
        let radius = 1.0;
        let mass_init_data = resources::MassInitData {
            inhabitable: true,
            motion: resources::MassMotion { position, velocity },
            color,
            radius,
        };
        init_data.masses.insert(mass_id, mass_init_data);
        mass_id += 1;
    }
    init_data
}

pub fn demo_2m2i() -> resources::InitData {
    let mut init_data = resources::InitData::default();
    let position = Vec3::X * 10.0;
    let velocity = Vec3::Y * 0.035;
    let radius = 1.0;
    init_data.masses.insert(
        0,
        resources::MassInitData {
            inhabitable: true,
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

pub fn demo_2m1i() -> resources::InitData {
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

pub fn demo_shooting() -> resources::InitData {
    let mut init_data = resources::InitData::default();
    let id_base = 0;

    let velocity = Vec3::ZERO;
    let color = Color::PURPLE;
    let x = 10.0;
    let y = 0.0;
    for i in 0..5 {
        let i = i * 4;
        let mass_id = id_base + i;
        let z = (i as f32 - 10.0) * 2.5;
        let position = Vec3::new(x, y, z);
        let motion = resources::MassMotion { position, velocity };
        let inhabitable = false;
        let radius = i as f32 / 5.0 + 1.0;
        init_data.masses.insert(
            mass_id,
            resources::MassInitData {
                inhabitable,
                motion,
                color,
                radius,
            },
        );
    }
    init_data.masses.insert(
        id_base + 21,
        resources::MassInitData {
            inhabitable: true,
            motion: resources::MassMotion {
                position: Vec3::new(-10.0, 0.0, 0.0),
                velocity: Vec3::ZERO,
            },
            color: Color::BLUE,
            radius: 3.0,
        },
    );
    init_data
}

/// Given a "latitude" and "longitude" on a unit sphere, return x,y,z
fn latlon_to_cartesian(lat: f32, lon: f32) -> Vec3 {
    let theta = (lat * 2.0 - 1.0).acos(); // latitude. -1 & 1 are poles. 0 is equator.
    let phi = lon * TAU; // portion around the sphere `[0,1)` (from Greenwich)
    let x = theta.sin() * phi.cos();
    let y = theta.sin() * phi.sin();
    let z = theta.cos();
    Vec3::new(x, y, z)
}

pub fn get_system(name: &str) -> impl (Fn() -> resources::InitData) {
    match name {
        "old_rando" => old_rando,
        "cubic" => cubic,
        "demo_2m2i" => demo_2m2i,
        "demo_2m1i" => demo_2m1i,
        "demo_shooting" => demo_shooting,
        _ => demo_2m1i,
    }
}
