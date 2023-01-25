use crate::*;
use bevy_rapier3d::prelude::{QueryFilter, RapierContext};
use std::collections::HashMap;

pub fn pimples() -> resources::InitData {
    let mut init_data = resources::InitData::default();
    let position = Vec3::X * 10.0;
    let velocity = Vec3::Y * 0.035;
    let mass = radius_to_mass(1.0);
    init_data.masses.insert(
        0,
        resources::MassInitData {
            inhabitable: false,
            motion: resources::MassMotion {
                position: position * 1.0,
                velocity: velocity * -1.0,
            },
            color: Color::RED,
            mass,
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
            mass,
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

/// `old_rando` used to be a "randomly generated" system. To remove the `rand` dependency,
/// the output was captured for a single run and here it is (in instantiation format).
pub fn old_rando() -> resources::InitData {
    resources::InitData {
        masses: HashMap::from([
            // Uninhabitable masses:
            (
                2033,
                resources::MassInitData {
                    inhabitable: false,
                    motion: resources::MassMotion {
                        position: Vec3::new(-38.18563, -16.365957, -14.895282),
                        velocity: Vec3::new(-0.97545135, -0.00109941, -0.12273329),
                    },
                    color: Color::rgba(0.7796566, 0.74294394, 0.55671966, 1.0),
                    mass: 98.040794,
                },
            ),
            (
                2008,
                resources::MassInitData {
                    inhabitable: false,
                    motion: resources::MassMotion {
                        position: Vec3::new(-21.527424, -5.7028694, 40.36987),
                        velocity: Vec3::new(-0.58495325, -0.003760232, 0.10443846),
                    },
                    color: Color::rgba(0.13856053, 0.5026426, 0.34816033, 1.0),
                    mass: 73.91596,
                },
            ),
            (
                2034,
                resources::MassInitData {
                    inhabitable: false,
                    motion: resources::MassMotion {
                        position: Vec3::new(-18.37727, -4.3796225, -46.20309),
                        velocity: Vec3::new(-0.027242454, -0.007021472, -0.6918575),
                    },
                    color: Color::rgba(0.4955992, 0.63004744, 0.16421562, 1.0),
                    mass: 86.547325,
                },
            ),
            (
                2022,
                resources::MassInitData {
                    inhabitable: false,
                    motion: resources::MassMotion {
                        position: Vec3::new(19.650904, -13.976096, -10.24036),
                        velocity: Vec3::new(-0.5919171, 0.0034584964, -0.26293072),
                    },
                    color: Color::rgba(0.11700261, 0.12954336, 0.8980697, 1.0),
                    mass: 126.329094,
                },
            ),
            (
                2000,
                resources::MassInitData {
                    inhabitable: false,
                    motion: resources::MassMotion {
                        position: Vec3::new(9.161188, -7.2353406, 1.0305831),
                        velocity: Vec3::new(0.8030496, -0.0044738017, -0.3907408),
                    },
                    color: Color::rgba(0.13821518, 0.99586564, 0.98818564, 1.0),
                    mass: 58.144608,
                },
            ),
            (
                2018,
                resources::MassInitData {
                    inhabitable: false,
                    motion: resources::MassMotion {
                        position: Vec3::new(-10.727993, 46.651474, 13.530999),
                        velocity: Vec3::new(-0.124982454, -0.008768366, -0.10915459),
                    },
                    color: Color::rgba(0.83886904, 0.3350374, 0.29212755, 1.0),
                    mass: 65.07727,
                },
            ),
            (
                2019,
                resources::MassInitData {
                    inhabitable: false,
                    motion: resources::MassMotion {
                        position: Vec3::new(10.727995, -46.651474, -13.530997),
                        velocity: Vec3::new(0.12498445, 0.008770366, 0.109156586),
                    },
                    color: Color::rgba(0.33108926, 0.9269361, 0.20500726, 1.0),
                    mass: 65.07727,
                },
            ),
            (
                2017,
                resources::MassInitData {
                    inhabitable: false,
                    motion: resources::MassMotion {
                        position: Vec3::new(5.091732, 23.021427, -4.2325954),
                        velocity: Vec3::new(0.40976635, -0.0018757978, 0.19066471),
                    },
                    color: Color::rgba(0.7039283, 0.14182454, 0.5862806, 1.0),
                    mass: 56.045044,
                },
            ),
            (
                2015,
                resources::MassInitData {
                    inhabitable: false,
                    motion: resources::MassMotion {
                        position: Vec3::new(5.190026, 0.27572125, 13.757441),
                        velocity: Vec3::new(-0.5740422, -0.0054757698, 0.5183668),
                    },
                    color: Color::rgba(0.4641121, 0.8153219, 0.27208978, 1.0),
                    mass: 58.649143,
                },
            ),
            (
                2025,
                resources::MassInitData {
                    inhabitable: false,
                    motion: resources::MassMotion {
                        position: Vec3::new(11.205822, 15.86477, 0.62063706),
                        velocity: Vec3::new(-0.6755305, 0.0011191271, 0.7205383),
                    },
                    color: Color::rgba(0.026519895, 0.040083468, 0.5605752, 1.0),
                    mass: 60.26995,
                },
            ),
            (
                2031,
                resources::MassInitData {
                    inhabitable: false,
                    motion: resources::MassMotion {
                        position: Vec3::new(16.953125, 21.460934, 0.22213744),
                        velocity: Vec3::new(-0.7418253, 0.0022850453, 0.6028883),
                    },
                    color: Color::rgba(0.90241665, 0.15674478, 0.94703704, 1.0),
                    mass: 61.794563,
                },
            ),
            (
                2005,
                resources::MassInitData {
                    inhabitable: false,
                    motion: resources::MassMotion {
                        position: Vec3::new(-14.441915, -0.65056473, 2.897615),
                        velocity: Vec3::new(0.6227617, 0.0007707861, -0.7314223),
                    },
                    color: Color::rgba(0.44703686, 0.5941764, 0.657763, 1.0),
                    mass: 80.885735,
                },
            ),
            // Inhabitable masses:
            (
                2037,
                resources::MassInitData {
                    inhabitable: true,
                    motion: resources::MassMotion {
                        position: Vec3::new(1e-6, 70.0, 1e-6),
                        velocity: Vec3::new(1e-6, 1e-6, 1e-6),
                    },
                    color: Color::rgba(17.0, 9.5, 46.0, 1.0),
                    mass: 4.712389,
                },
            ),
            (
                2001,
                resources::MassInitData {
                    inhabitable: false,
                    motion: resources::MassMotion {
                        position: Vec3::new(-9.161186, 7.2353425, -1.0305812),
                        velocity: Vec3::new(-0.8030476, 0.0044758013, 0.39074284),
                    },
                    color: Color::rgba(0.2364595, 0.26930845, 0.64689803, 1.0),
                    mass: 58.144608,
                },
            ),
            (
                2024,
                resources::MassInitData {
                    inhabitable: false,
                    motion: resources::MassMotion {
                        position: Vec3::new(-11.20582, -15.864768, -0.62063503),
                        velocity: Vec3::new(0.6755325, -0.0011171271, -0.7205363),
                    },
                    color: Color::rgba(0.8483983, 0.013288438, 0.7639806, 1.0),
                    mass: 60.26995,
                },
            ),
            (
                2006,
                resources::MassInitData {
                    inhabitable: false,
                    motion: resources::MassMotion {
                        position: Vec3::new(22.042305, -30.1111, 17.284887),
                        velocity: Vec3::new(0.90640867, 0.0037647518, -0.06767071),
                    },
                    color: Color::rgba(0.8634984, 0.35339612, 0.8521633, 1.0),
                    mass: 57.71097,
                },
            ),
            (
                2038,
                resources::MassInitData {
                    inhabitable: true,
                    motion: resources::MassMotion {
                        position: Vec3::new(1e-6, 1e-6, 70.0),
                        velocity: Vec3::new(1e-6, 1e-6, 1e-6),
                    },
                    color: Color::rgba(17.0, 6.3333335, 69.0, 1.0),
                    mass: 4.712389,
                },
            ),
            (
                2016,
                resources::MassInitData {
                    inhabitable: false,
                    motion: resources::MassMotion {
                        position: Vec3::new(-5.09173, -23.021423, 4.2325974),
                        velocity: Vec3::new(-0.40976432, 0.0018777978, -0.19066271),
                    },
                    color: Color::rgba(0.10967934, 0.9698939, 0.27716374, 1.0),
                    mass: 56.045044,
                },
            ),
            (
                2003,
                resources::MassInitData {
                    inhabitable: false,
                    motion: resources::MassMotion {
                        position: Vec3::new(-16.988888, 10.84331, 44.61658),
                        velocity: Vec3::new(0.44792792, -0.0051308344, -0.5568966),
                    },
                    color: Color::rgba(0.47933078, 0.37206656, 0.0052149296, 1.0),
                    mass: 85.25729,
                },
            ),
            (
                2020,
                resources::MassInitData {
                    inhabitable: false,
                    motion: resources::MassMotion {
                        position: Vec3::new(47.31852, 13.543327, -0.72530705),
                        velocity: Vec3::new(0.7601094, -0.0016827884, -0.57057023),
                    },
                    color: Color::rgba(0.01097244, 0.49121875, 0.32062954, 1.0),
                    mass: 60.23066,
                },
            ),
            (
                2009,
                resources::MassInitData {
                    inhabitable: false,
                    motion: resources::MassMotion {
                        position: Vec3::new(21.527428, 5.7028713, -40.36987),
                        velocity: Vec3::new(0.5849553, 0.003762232, -0.104436465),
                    },
                    color: Color::rgba(0.006866038, 0.6088672, 0.20898598, 1.0),
                    mass: 73.91596,
                },
            ),
            (
                2013,
                resources::MassInitData {
                    inhabitable: false,
                    motion: resources::MassMotion {
                        position: Vec3::new(-2.2454338, -13.705887, -21.335745),
                        velocity: Vec3::new(-0.5670095, 0.0007114005, 0.81629264),
                    },
                    color: Color::rgba(0.22836238, 0.41734862, 0.38657147, 1.0),
                    mass: 75.07582,
                },
            ),
            (
                2002,
                resources::MassInitData {
                    inhabitable: false,
                    motion: resources::MassMotion {
                        position: Vec3::new(16.988892, -10.843308, -44.61658),
                        velocity: Vec3::new(-0.4479259, 0.005132834, 0.55689865),
                    },
                    color: Color::rgba(0.5667388, 0.4170677, 0.97128177, 1.0),
                    mass: 85.25729,
                },
            ),
            (
                2026,
                resources::MassInitData {
                    inhabitable: false,
                    motion: resources::MassMotion {
                        position: Vec3::new(-12.535445, -1.6048312, -1.9671205),
                        velocity: Vec3::new(-0.8018609, 0.00037506176, 0.58809537),
                    },
                    color: Color::rgba(0.09541446, 0.1797868, 0.30913943, 1.0),
                    mass: 99.60823,
                },
            ),
            (
                2027,
                resources::MassInitData {
                    inhabitable: false,
                    motion: resources::MassMotion {
                        position: Vec3::new(12.535447, 1.6048331, 1.9671224),
                        velocity: Vec3::new(0.80186296, -0.00037306175, -0.58809334),
                    },
                    color: Color::rgba(0.15984082, 0.03131342, 0.05979371, 1.0),
                    mass: 99.60823,
                },
            ),
            (
                2028,
                resources::MassInitData {
                    inhabitable: false,
                    motion: resources::MassMotion {
                        position: Vec3::new(25.86586, -27.072681, 28.040224),
                        velocity: Vec3::new(-0.61429983, 0.0031392337, -0.06214477),
                    },
                    color: Color::rgba(0.8846535, 0.045195162, 0.31272066, 1.0),
                    mass: 108.44107,
                },
            ),
            (
                2035,
                resources::MassInitData {
                    inhabitable: false,
                    motion: resources::MassMotion {
                        position: Vec3::new(18.377274, 4.3796244, 46.20309),
                        velocity: Vec3::new(0.027244454, 0.0070234714, 0.69185954),
                    },
                    color: Color::rgba(0.088169515, 0.1519646, 0.7791658, 1.0),
                    mass: 86.547325,
                },
            ),
            (
                2012,
                resources::MassInitData {
                    inhabitable: false,
                    motion: resources::MassMotion {
                        position: Vec3::new(2.2454357, 13.705889, 21.335749),
                        velocity: Vec3::new(0.56701154, -0.0007094005, -0.8162906),
                    },
                    color: Color::rgba(0.094447196, 0.100011885, 0.7901554, 1.0),
                    mass: 75.07582,
                },
            ),
            (
                2036,
                resources::MassInitData {
                    inhabitable: true,
                    motion: resources::MassMotion {
                        position: Vec3::new(70.0, 1e-6, 1e-6),
                        velocity: Vec3::new(1e-6, 1e-6, 1e-6),
                    },
                    color: Color::rgba(17.0, 19.0, 23.0, 1.0),
                    mass: 4.712389,
                },
            ),
            (
                2014,
                resources::MassInitData {
                    inhabitable: false,
                    motion: resources::MassMotion {
                        position: Vec3::new(-5.190024, -0.27571923, -13.757439),
                        velocity: Vec3::new(0.5740442, 0.0054777693, -0.5183648),
                    },
                    color: Color::rgba(0.34330362, 0.7357522, 0.58593976, 1.0),
                    mass: 58.649143,
                },
            ),
            (
                2030,
                resources::MassInitData {
                    inhabitable: false,
                    motion: resources::MassMotion {
                        position: Vec3::new(-16.953121, -21.46093, -0.22213544),
                        velocity: Vec3::new(0.7418273, -0.0022830453, -0.60288626),
                    },
                    color: Color::rgba(0.44308203, 0.04808724, 0.9348498, 1.0),
                    mass: 61.794563,
                },
            ),
            (
                2010,
                resources::MassInitData {
                    inhabitable: false,
                    motion: resources::MassMotion {
                        position: Vec3::new(16.930954, 29.013283, 15.770469),
                        velocity: Vec3::new(0.720379, 0.00070454687, 0.4428605),
                    },
                    color: Color::rgba(0.9598586, 0.42036194, 0.29315, 1.0),
                    mass: 49.802612,
                },
            ),
            (
                2007,
                resources::MassInitData {
                    inhabitable: false,
                    motion: resources::MassMotion {
                        position: Vec3::new(-22.042301, 30.111103, -17.284883),
                        velocity: Vec3::new(-0.90640664, -0.0037627518, 0.06767271),
                    },
                    color: Color::rgba(0.6276091, 0.10356903, 0.95841837, 1.0),
                    mass: 57.71097,
                },
            ),
            (
                2029,
                resources::MassInitData {
                    inhabitable: false,
                    motion: resources::MassMotion {
                        position: Vec3::new(-25.865856, 27.072685, -28.04022),
                        velocity: Vec3::new(0.61430186, -0.0031372337, 0.062146768),
                    },
                    color: Color::rgba(0.36995202, 0.5972635, 0.64231443, 1.0),
                    mass: 108.44107,
                },
            ),
            (
                2004,
                resources::MassInitData {
                    inhabitable: false,
                    motion: resources::MassMotion {
                        position: Vec3::new(14.441916, 0.65056676, -2.897613),
                        velocity: Vec3::new(-0.6227597, -0.00076878606, 0.73142433),
                    },
                    color: Color::rgba(0.5996596, 0.30508322, 0.45769888, 1.0),
                    mass: 80.885735,
                },
            ),
            (
                2032,
                resources::MassInitData {
                    inhabitable: false,
                    motion: resources::MassMotion {
                        position: Vec3::new(38.18563, 16.365961, 14.895284),
                        velocity: Vec3::new(0.9754534, 0.00110141, 0.122735284),
                    },
                    color: Color::rgba(0.45787936, 0.8522816, 0.20645785, 1.0),
                    mass: 98.040794,
                },
            ),
            (
                2023,
                resources::MassInitData {
                    inhabitable: false,
                    motion: resources::MassMotion {
                        position: Vec3::new(-19.6509, 13.976098, 10.240362),
                        velocity: Vec3::new(0.5919191, -0.0034564964, 0.26293275),
                    },
                    color: Color::rgba(0.3148377, 0.62697864, 0.9439282, 1.0),
                    mass: 126.329094,
                },
            ),
            (
                2011,
                resources::MassInitData {
                    inhabitable: false,
                    motion: resources::MassMotion {
                        position: Vec3::new(-16.93095, -29.013279, -15.770467),
                        velocity: Vec3::new(-0.72037697, -0.00070254685, -0.4428585),
                    },
                    color: Color::rgba(0.46598637, 0.31868017, 0.2907951, 1.0),
                    mass: 49.802612,
                },
            ),
            (
                2021,
                resources::MassInitData {
                    inhabitable: false,
                    motion: resources::MassMotion {
                        position: Vec3::new(-47.31852, -13.543325, 0.7253091),
                        velocity: Vec3::new(-0.7601074, 0.0016847884, 0.57057226),
                    },
                    color: Color::rgba(0.654255, 0.52485174, 0.22863472, 1.0),
                    mass: 60.23066,
                },
            ),
        ]),
    }
}
