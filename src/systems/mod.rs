use crate::{networking::*, physics::Momentum, radius_to_mass, PointMassBundle};
use bevy::prelude::*;
use bevy_rapier3d::prelude::Collider;
use rand::Rng;
use std::f32::consts::TAU;

/// Old rando from way back
pub fn old_rando(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut planet_data: ResMut<InitData>,
) {
    let mut rng = rand::thread_rng();
    let mut rf = || rng.gen::<f32>();
    let pair_count = 18;
    let mut planet_id = 2000;
    for _ in 0..pair_count {
        let position = latlon_to_cartesian(rf(), rf()) * (rf() * 40.0 + 10.0);
        let velocity = latlon_to_cartesian(rf(), rf()) * Vec3::new(10.0, rf() * 0.1, 10.0) * 0.1;
        let radius = rf() + 2.0;
        for side in [-1.0, 1.0] {
            let color = Color::rgb(rf(), rf(), rf());
            let position = position * side;
            let velocity = velocity * side;
            let planet_init_data = PlanetInitData {
                position,
                velocity,
                color,
                radius,
            };
            planet_data.planets.insert(planet_id, planet_init_data);
            planet_id += 1;
            spawn_planet(
                planet_id,
                planet_init_data,
                &mut commands,
                &mut meshes,
                &mut materials,
            );
        }
    }
}

/// Make some interesting planets
pub fn cubic(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut planet_data: ResMut<InitData>,
) {
    let mut planet_id = 2000;
    let radius = 0.5;
    let from_origin = 9.0;
    for n in [(1, 0, 0), (0, 1, 0), (0, 0, 1)] {
        for side in [1.0, -1.0] {
            let fun_factor = 1.0 + (planet_id as f32 - 2000.0) / 20.0;

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

            let planet_init_data = PlanetInitData {
                position,
                velocity,
                color,
                radius,
            };
            planet_data.planets.insert(planet_id, planet_init_data);
            planet_id += 1;
            spawn_planet(
                planet_id,
                planet_init_data,
                &mut commands,
                &mut meshes,
                &mut materials,
            );
        }
    }
}

pub fn spawn_planet<'a>(
    planet_id: u64,
    planet_init_data: PlanetInitData,
    commands: &'a mut Commands,
    meshes: &'a mut ResMut<Assets<Mesh>>,
    materials: &'a mut ResMut<Assets<StandardMaterial>>,
) -> Entity {
    let PlanetInitData {
        position,
        velocity,
        color,
        radius,
    } = planet_init_data;
    commands
        .spawn(PointMassBundle {
            pbr: PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Icosphere {
                    radius,
                    ..Default::default()
                })),
                material: materials.add(color.into()),
                transform: Transform::from_translation(position),
                ..Default::default()
            },
            momentum: Momentum {
                velocity,
                mass: radius_to_mass(radius),
                ..Default::default()
            },
            collider: Collider::ball(radius),
            ..Default::default()
        })
        .insert(MassID(planet_id))
        .id()
}

fn latlon_to_cartesian(lat: f32, lon: f32) -> Vec3 {
    let theta = (lat * 2.0 - 1.0).acos(); // latitude. -1 & 1 are poles. 0 is equator.
    let phi = lon * TAU; // portion around the planet `[0,1)` (from Greenwich)
    let x = theta.sin() * phi.cos();
    let y = theta.sin() * phi.sin();
    let z = theta.cos();
    Vec3::new(x, y, z)
}
