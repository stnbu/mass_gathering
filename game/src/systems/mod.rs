/// Code for generating the various hand-built systems.
///
/// NOTE: I don't like "system" but here I mean roughly "solar system" not "bevy system", please post your ideas to my coordinates.
///
/// The way I think this should work: A separate package (?) provides tools to build serialized "systems" which you may write to disk if you like.
/// The game can just somehow consume the serialized version, unpack, and proceed as before.
use crate::*;
use std::collections::HashMap;

/// refactor_tags: system, to_client_read, to_client_write, simulation, refactor
pub fn demo_2m2i() -> resources::InitData {
    let mut init_data = resources::InitData::default();
    let position = Vec3::X * 10.0;
    let velocity = Vec3::Y * 0.035;
    let mass = radius_to_mass(1.0);
    init_data.masses.insert(
        0,
        resources::MassInitData {
            inhabitation: components::Inhabitation::Inhabitable(None),
            motion: resources::MassMotion {
                position: position * 1.0,
                velocity: velocity * -1.0,
            },
            color: [1.0, 0.0, 0.0, 1.0],
            mass,
        },
    );
    init_data.masses.insert(
        1,
        resources::MassInitData {
            inhabitation: components::Inhabitation::Inhabitable(None),
            motion: resources::MassMotion {
                position: position * -1.0,
                velocity: velocity * 1.0,
            },
            color: [0.0, 0.0, 1.0, 1.0],
            mass,
        },
    );
    init_data
}

/// refactor_tags: system, to_client_read, to_client_write, simulation, refactor
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
        let inhabitation = components::Inhabitation::Uninhabitable;
        let mass = radius_to_mass(i as f32 / 5.0 + 1.0);
        let color = [1.0, 0.0, 0.0, 1.0];
        init_data.masses.insert(
            mass_id,
            resources::MassInitData {
                inhabitation,
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
            inhabitation: components::Inhabitation::Inhabitable(None),
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

/// refactor_tags: system, to_client_read, to_client_write, simulation, refactor
pub fn rando_calrissian() -> resources::InitData {
    resources::InitData {
        masses: HashMap::from([
            // Uninhabitable masses:
            (
                2033,
                resources::MassInitData {
                    inhabitation: components::Inhabitation::Uninhabitable,
                    motion: resources::MassMotion {
                        position: Vec3::new(-38.18563, -16.365957, -14.895282),
                        velocity: Vec3::new(-0.97545135, -0.00109941, -0.12273329),
                    },
                    color: [0.7796566, 0.74294394, 0.55671966, 1.0],
                    mass: 98.040794,
                },
            ),
            (
                2008,
                resources::MassInitData {
                    inhabitation: components::Inhabitation::Uninhabitable,
                    motion: resources::MassMotion {
                        position: Vec3::new(-21.527424, -5.7028694, 40.36987),
                        velocity: Vec3::new(-0.58495325, -0.003760232, 0.10443846),
                    },
                    color: [0.13856053, 0.5026426, 0.34816033, 1.0],
                    mass: 73.91596,
                },
            ),
            (
                2034,
                resources::MassInitData {
                    inhabitation: components::Inhabitation::Uninhabitable,
                    motion: resources::MassMotion {
                        position: Vec3::new(-18.37727, -4.3796225, -46.20309),
                        velocity: Vec3::new(-0.027242454, -0.007021472, -0.6918575),
                    },
                    color: [0.4955992, 0.63004744, 0.16421562, 1.0],
                    mass: 86.547325,
                },
            ),
            (
                2022,
                resources::MassInitData {
                    inhabitation: components::Inhabitation::Uninhabitable,
                    motion: resources::MassMotion {
                        position: Vec3::new(19.650904, -13.976096, -10.24036),
                        velocity: Vec3::new(-0.5919171, 0.0034584964, -0.26293072),
                    },
                    color: [0.11700261, 0.12954336, 0.8980697, 1.0],
                    mass: 126.329094,
                },
            ),
            (
                2000,
                resources::MassInitData {
                    inhabitation: components::Inhabitation::Uninhabitable,
                    motion: resources::MassMotion {
                        position: Vec3::new(9.161188, -7.2353406, 1.0305831),
                        velocity: Vec3::new(0.8030496, -0.0044738017, -0.3907408),
                    },
                    color: [0.13821518, 0.99586564, 0.98818564, 1.0],
                    mass: 58.144608,
                },
            ),
            (
                2018,
                resources::MassInitData {
                    inhabitation: components::Inhabitation::Uninhabitable,
                    motion: resources::MassMotion {
                        position: Vec3::new(-10.727993, 46.651474, 13.530999),
                        velocity: Vec3::new(-0.124982454, -0.008768366, -0.10915459),
                    },
                    color: [0.83886904, 0.3350374, 0.29212755, 1.0],
                    mass: 65.07727,
                },
            ),
            (
                2019,
                resources::MassInitData {
                    inhabitation: components::Inhabitation::Uninhabitable,
                    motion: resources::MassMotion {
                        position: Vec3::new(10.727995, -46.651474, -13.530997),
                        velocity: Vec3::new(0.12498445, 0.008770366, 0.109156586),
                    },
                    color: [0.33108926, 0.9269361, 0.20500726, 1.0],
                    mass: 65.07727,
                },
            ),
            (
                2017,
                resources::MassInitData {
                    inhabitation: components::Inhabitation::Uninhabitable,
                    motion: resources::MassMotion {
                        position: Vec3::new(5.091732, 23.021427, -4.2325954),
                        velocity: Vec3::new(0.40976635, -0.0018757978, 0.19066471),
                    },
                    color: [0.7039283, 0.14182454, 0.5862806, 1.0],
                    mass: 56.045044,
                },
            ),
            (
                2015,
                resources::MassInitData {
                    inhabitation: components::Inhabitation::Uninhabitable,
                    motion: resources::MassMotion {
                        position: Vec3::new(5.190026, 0.27572125, 13.757441),
                        velocity: Vec3::new(-0.5740422, -0.0054757698, 0.5183668),
                    },
                    color: [0.4641121, 0.8153219, 0.27208978, 1.0],
                    mass: 58.649143,
                },
            ),
            (
                2025,
                resources::MassInitData {
                    inhabitation: components::Inhabitation::Uninhabitable,
                    motion: resources::MassMotion {
                        position: Vec3::new(11.205822, 15.86477, 0.62063706),
                        velocity: Vec3::new(-0.6755305, 0.0011191271, 0.7205383),
                    },
                    color: [0.026519895, 0.040083468, 0.5605752, 1.0],
                    mass: 60.26995,
                },
            ),
            (
                2031,
                resources::MassInitData {
                    inhabitation: components::Inhabitation::Uninhabitable,
                    motion: resources::MassMotion {
                        position: Vec3::new(16.953125, 21.460934, 0.22213744),
                        velocity: Vec3::new(-0.7418253, 0.0022850453, 0.6028883),
                    },
                    color: [0.90241665, 0.15674478, 0.94703704, 1.0],
                    mass: 61.794563,
                },
            ),
            (
                2005,
                resources::MassInitData {
                    inhabitation: components::Inhabitation::Uninhabitable,
                    motion: resources::MassMotion {
                        position: Vec3::new(-14.441915, -0.65056473, 2.897615),
                        velocity: Vec3::new(0.6227617, 0.0007707861, -0.7314223),
                    },
                    color: [0.44703686, 0.5941764, 0.657763, 1.0],
                    mass: 80.885735,
                },
            ),
            // Inhabitable masses:
            (
                2037,
                resources::MassInitData {
                    inhabitation: components::Inhabitation::Inhabitable(None),
                    motion: resources::MassMotion {
                        position: Vec3::new(1e-6, 70.0, 1e-6),
                        velocity: Vec3::new(1e-6, 1e-6, 1e-6),
                    },
                    color: [17.0, 9.5, 46.0, 1.0],
                    mass: 4.712389,
                },
            ),
            (
                2001,
                resources::MassInitData {
                    inhabitation: components::Inhabitation::Uninhabitable,
                    motion: resources::MassMotion {
                        position: Vec3::new(-9.161186, 7.2353425, -1.0305812),
                        velocity: Vec3::new(-0.8030476, 0.0044758013, 0.39074284),
                    },
                    color: [0.2364595, 0.26930845, 0.64689803, 1.0],
                    mass: 58.144608,
                },
            ),
            (
                2024,
                resources::MassInitData {
                    inhabitation: components::Inhabitation::Uninhabitable,
                    motion: resources::MassMotion {
                        position: Vec3::new(-11.20582, -15.864768, -0.62063503),
                        velocity: Vec3::new(0.6755325, -0.0011171271, -0.7205363),
                    },
                    color: [0.8483983, 0.013288438, 0.7639806, 1.0],
                    mass: 60.26995,
                },
            ),
            (
                2006,
                resources::MassInitData {
                    inhabitation: components::Inhabitation::Uninhabitable,
                    motion: resources::MassMotion {
                        position: Vec3::new(22.042305, -30.1111, 17.284887),
                        velocity: Vec3::new(0.90640867, 0.0037647518, -0.06767071),
                    },
                    color: [0.8634984, 0.35339612, 0.8521633, 1.0],
                    mass: 57.71097,
                },
            ),
            (
                2038,
                resources::MassInitData {
                    inhabitation: components::Inhabitation::Inhabitable(None),
                    motion: resources::MassMotion {
                        position: Vec3::new(1e-6, 1e-6, 70.0),
                        velocity: Vec3::new(1e-6, 1e-6, 1e-6),
                    },
                    color: [17.0, 6.3333335, 69.0, 1.0],
                    mass: 4.712389,
                },
            ),
            (
                2016,
                resources::MassInitData {
                    inhabitation: components::Inhabitation::Uninhabitable,
                    motion: resources::MassMotion {
                        position: Vec3::new(-5.09173, -23.021423, 4.2325974),
                        velocity: Vec3::new(-0.40976432, 0.0018777978, -0.19066271),
                    },
                    color: [0.10967934, 0.9698939, 0.27716374, 1.0],
                    mass: 56.045044,
                },
            ),
            (
                2003,
                resources::MassInitData {
                    inhabitation: components::Inhabitation::Uninhabitable,
                    motion: resources::MassMotion {
                        position: Vec3::new(-16.988888, 10.84331, 44.61658),
                        velocity: Vec3::new(0.44792792, -0.0051308344, -0.5568966),
                    },
                    color: [0.47933078, 0.37206656, 0.0052149296, 1.0],
                    mass: 85.25729,
                },
            ),
            (
                2020,
                resources::MassInitData {
                    inhabitation: components::Inhabitation::Uninhabitable,
                    motion: resources::MassMotion {
                        position: Vec3::new(47.31852, 13.543327, -0.72530705),
                        velocity: Vec3::new(0.7601094, -0.0016827884, -0.57057023),
                    },
                    color: [0.01097244, 0.49121875, 0.32062954, 1.0],
                    mass: 60.23066,
                },
            ),
            (
                2009,
                resources::MassInitData {
                    inhabitation: components::Inhabitation::Uninhabitable,
                    motion: resources::MassMotion {
                        position: Vec3::new(21.527428, 5.7028713, -40.36987),
                        velocity: Vec3::new(0.5849553, 0.003762232, -0.104436465),
                    },
                    color: [0.006866038, 0.6088672, 0.20898598, 1.0],
                    mass: 73.91596,
                },
            ),
            (
                2013,
                resources::MassInitData {
                    inhabitation: components::Inhabitation::Uninhabitable,
                    motion: resources::MassMotion {
                        position: Vec3::new(-2.2454338, -13.705887, -21.335745),
                        velocity: Vec3::new(-0.5670095, 0.0007114005, 0.81629264),
                    },
                    color: [0.22836238, 0.41734862, 0.38657147, 1.0],
                    mass: 75.07582,
                },
            ),
            (
                2002,
                resources::MassInitData {
                    inhabitation: components::Inhabitation::Uninhabitable,
                    motion: resources::MassMotion {
                        position: Vec3::new(16.988892, -10.843308, -44.61658),
                        velocity: Vec3::new(-0.4479259, 0.005132834, 0.55689865),
                    },
                    color: [0.5667388, 0.4170677, 0.97128177, 1.0],
                    mass: 85.25729,
                },
            ),
            (
                2026,
                resources::MassInitData {
                    inhabitation: components::Inhabitation::Uninhabitable,
                    motion: resources::MassMotion {
                        position: Vec3::new(-12.535445, -1.6048312, -1.9671205),
                        velocity: Vec3::new(-0.8018609, 0.00037506176, 0.58809537),
                    },
                    color: [0.09541446, 0.1797868, 0.30913943, 1.0],
                    mass: 99.60823,
                },
            ),
            (
                2027,
                resources::MassInitData {
                    inhabitation: components::Inhabitation::Uninhabitable,
                    motion: resources::MassMotion {
                        position: Vec3::new(12.535447, 1.6048331, 1.9671224),
                        velocity: Vec3::new(0.80186296, -0.00037306175, -0.58809334),
                    },
                    color: [0.15984082, 0.03131342, 0.05979371, 1.0],
                    mass: 99.60823,
                },
            ),
            (
                2028,
                resources::MassInitData {
                    inhabitation: components::Inhabitation::Uninhabitable,
                    motion: resources::MassMotion {
                        position: Vec3::new(25.86586, -27.072681, 28.040224),
                        velocity: Vec3::new(-0.61429983, 0.0031392337, -0.06214477),
                    },
                    color: [0.8846535, 0.045195162, 0.31272066, 1.0],
                    mass: 108.44107,
                },
            ),
            (
                2035,
                resources::MassInitData {
                    inhabitation: components::Inhabitation::Uninhabitable,
                    motion: resources::MassMotion {
                        position: Vec3::new(18.377274, 4.3796244, 46.20309),
                        velocity: Vec3::new(0.027244454, 0.0070234714, 0.69185954),
                    },
                    color: [0.088169515, 0.1519646, 0.7791658, 1.0],
                    mass: 86.547325,
                },
            ),
            (
                2012,
                resources::MassInitData {
                    inhabitation: components::Inhabitation::Uninhabitable,
                    motion: resources::MassMotion {
                        position: Vec3::new(2.2454357, 13.705889, 21.335749),
                        velocity: Vec3::new(0.56701154, -0.0007094005, -0.8162906),
                    },
                    color: [0.094447196, 0.100011885, 0.7901554, 1.0],
                    mass: 75.07582,
                },
            ),
            (
                2036,
                resources::MassInitData {
                    inhabitation: components::Inhabitation::Inhabitable(None),
                    motion: resources::MassMotion {
                        position: Vec3::new(70.0, 1e-6, 1e-6),
                        velocity: Vec3::new(1e-6, 1e-6, 1e-6),
                    },
                    color: [17.0, 19.0, 23.0, 1.0],
                    mass: 4.712389,
                },
            ),
            (
                2014,
                resources::MassInitData {
                    inhabitation: components::Inhabitation::Uninhabitable,
                    motion: resources::MassMotion {
                        position: Vec3::new(-5.190024, -0.27571923, -13.757439),
                        velocity: Vec3::new(0.5740442, 0.0054777693, -0.5183648),
                    },
                    color: [0.34330362, 0.7357522, 0.58593976, 1.0],
                    mass: 58.649143,
                },
            ),
            (
                2030,
                resources::MassInitData {
                    inhabitation: components::Inhabitation::Uninhabitable,
                    motion: resources::MassMotion {
                        position: Vec3::new(-16.953121, -21.46093, -0.22213544),
                        velocity: Vec3::new(0.7418273, -0.0022830453, -0.60288626),
                    },
                    color: [0.44308203, 0.04808724, 0.9348498, 1.0],
                    mass: 61.794563,
                },
            ),
            (
                2010,
                resources::MassInitData {
                    inhabitation: components::Inhabitation::Uninhabitable,
                    motion: resources::MassMotion {
                        position: Vec3::new(16.930954, 29.013283, 15.770469),
                        velocity: Vec3::new(0.720379, 0.00070454687, 0.4428605),
                    },
                    color: [0.9598586, 0.42036194, 0.29315, 1.0],
                    mass: 49.802612,
                },
            ),
            (
                2007,
                resources::MassInitData {
                    inhabitation: components::Inhabitation::Uninhabitable,
                    motion: resources::MassMotion {
                        position: Vec3::new(-22.042301, 30.111103, -17.284883),
                        velocity: Vec3::new(-0.90640664, -0.0037627518, 0.06767271),
                    },
                    color: [0.6276091, 0.10356903, 0.95841837, 1.0],
                    mass: 57.71097,
                },
            ),
            (
                2029,
                resources::MassInitData {
                    inhabitation: components::Inhabitation::Uninhabitable,
                    motion: resources::MassMotion {
                        position: Vec3::new(-25.865856, 27.072685, -28.04022),
                        velocity: Vec3::new(0.61430186, -0.0031372337, 0.062146768),
                    },
                    color: [0.36995202, 0.5972635, 0.64231443, 1.0],
                    mass: 108.44107,
                },
            ),
            (
                2004,
                resources::MassInitData {
                    inhabitation: components::Inhabitation::Uninhabitable,
                    motion: resources::MassMotion {
                        position: Vec3::new(14.441916, 0.65056676, -2.897613),
                        velocity: Vec3::new(-0.6227597, -0.00076878606, 0.73142433),
                    },
                    color: [0.5996596, 0.30508322, 0.45769888, 1.0],
                    mass: 80.885735,
                },
            ),
            (
                2032,
                resources::MassInitData {
                    inhabitation: components::Inhabitation::Uninhabitable,
                    motion: resources::MassMotion {
                        position: Vec3::new(38.18563, 16.365961, 14.895284),
                        velocity: Vec3::new(0.9754534, 0.00110141, 0.122735284),
                    },
                    color: [0.45787936, 0.8522816, 0.20645785, 1.0],
                    mass: 98.040794,
                },
            ),
            (
                2023,
                resources::MassInitData {
                    inhabitation: components::Inhabitation::Uninhabitable,
                    motion: resources::MassMotion {
                        position: Vec3::new(-19.6509, 13.976098, 10.240362),
                        velocity: Vec3::new(0.5919191, -0.0034564964, 0.26293275),
                    },
                    color: [0.3148377, 0.62697864, 0.9439282, 1.0],
                    mass: 126.329094,
                },
            ),
            (
                2011,
                resources::MassInitData {
                    inhabitation: components::Inhabitation::Uninhabitable,
                    motion: resources::MassMotion {
                        position: Vec3::new(-16.93095, -29.013279, -15.770467),
                        velocity: Vec3::new(-0.72037697, -0.00070254685, -0.4428585),
                    },
                    color: [0.46598637, 0.31868017, 0.2907951, 1.0],
                    mass: 49.802612,
                },
            ),
            (
                2021,
                resources::MassInitData {
                    inhabitation: components::Inhabitation::Uninhabitable,
                    motion: resources::MassMotion {
                        position: Vec3::new(-47.31852, -13.543325, 0.7253091),
                        velocity: Vec3::new(-0.7601074, 0.0016847884, 0.57057226),
                    },
                    color: [0.654255, 0.52485174, 0.22863472, 1.0],
                    mass: 60.23066,
                },
            ),
        ]),
    }
}

/// refactor_tags: system, to_client_read, to_client_write, simulation, refactor
pub fn cubic() -> resources::InitData {
    let mut init_data = resources::InitData::default();

    let mut mass_id = 2000;
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
                [r, g, b, 0.8]
            } else {
                [(1.0 - r) / 2.0, (1.0 - g) / 2.0, (1.0 - b) / 2.0, 0.8]
            };
            let velocity = if c == 1 {
                velocity
            } else {
                velocity * fun_factor
            };
            let mass = radius_to_mass(if a == 1 { 0.5 } else { 0.5 * fun_factor });

            let position = if c == 1 {
                position
            } else {
                position * fun_factor
            };

            let mass_init_data = resources::MassInitData {
                inhabitation: components::Inhabitation::Uninhabitable,
                motion: resources::MassMotion { position, velocity },
                color,
                mass,
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
        let color = [17.0, 19.0 / color_tweak, 23.0 * color_tweak, 1.0];
        let mass = radius_to_mass(1.0);
        let inhabitation = if z == 1 {
            components::Inhabitation::Inhabitable(None)
        } else {
            components::Inhabitation::Uninhabitable
        };
        let mass_init_data = resources::MassInitData {
            inhabitation,
            motion: resources::MassMotion { position, velocity },
            color,
            mass,
        };
        init_data.masses.insert(mass_id, mass_init_data);
        mass_id += 1;
    }
    init_data
}

/// refactor_tags: system, to_client_read, to_client_write, simulation, refactor
pub fn get_system(name: &str) -> impl (Fn() -> resources::InitData) {
    match name {
        "cubic" => cubic,
        "rando_calrissian" => rando_calrissian,
        "demo_2m2i" => demo_2m2i,
        "demo_shooting" => demo_shooting,
        _ => panic!("No such system: {name}"),
    }
}
