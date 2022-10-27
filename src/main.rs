use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_rapier3d::prelude::{
    ActiveEvents, Collider, CollisionEvent, NoUserData, QueryFilter, RapierConfiguration,
    RapierContext, RapierPhysicsPlugin, RigidBody, Sensor,
};
use rand::Rng;
use std::collections::HashSet;
use std::f32::consts::TAU;

mod physics;
use physics::*;

mod craft;
use craft::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::MIDNIGHT_BLUE * 0.1))
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_state(AppState::Startup)
        .add_system_set(
            SystemSet::on_update(AppState::Playing)
                .with_system(move_forward)
                .with_system(steer)
                .with_system(freefall)
                //.with_system(collision_events)
                .with_system(handle_projectile_engagement)
                .with_system(handle_projectile_flight),
        )
        .add_startup_system(setup)
        .add_startup_system(spacecraft_setup)
        .add_system(bevy::window::close_on_esc)
        .add_system(handle_game_state)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_system(hud)
        .run();
}

#[derive(Component)]
struct BallisticProjectileTarget {
    planet: Entity,
    local_impact_site: Vec3,
}

fn handle_projectile_flight(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut projectile_query: Query<(Entity, &mut Transform, &BallisticProjectileTarget)>,
    planet_query: Query<
        (&Transform, &Momentum),
        (With<Collider>, Without<BallisticProjectileTarget>),
    >,
    mut collision_events: EventReader<CollisionEvent>,
    time: Res<Time>,
) {
    let mut collided = HashSet::new();
    for event in collision_events.iter() {
        if let CollisionEvent::Started(e0, e1, _) = event {
            collided.insert(e0);
            collided.insert(e1);
        }
        if let CollisionEvent::Stopped(e0, e1, _) = event {
            collided.insert(e0);
            collided.insert(e1);
        }
    }

    for (projectile, mut projectile_transform, target) in projectile_query.iter_mut() {
        if collided.contains(&&projectile) {
            let explosion = commands
                .spawn_bundle(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Icosphere {
                        radius: 0.2,
                        ..Default::default()
                    })),
                    material: materials.add(Color::YELLOW.into()),
                    transform: Transform::from_translation(target.local_impact_site),
                    ..Default::default()
                })
                .id();
            commands.entity(target.planet).push_children(&[explosion]);
            info!("despawning {:?}", projectile);
            commands.entity(projectile).despawn();
            continue;
        }
        if let Ok((planet_transform, planet_momentum)) = planet_query.get(target.planet) {
            let goal_impact_site = planet_transform.translation + target.local_impact_site;
            let direction = (projectile_transform.translation - goal_impact_site).normalize();
            projectile_transform.translation -=
                (direction + (planet_momentum.velocity * time.delta_seconds() * 0.8)) * 0.4;
        }
    }
}

fn handle_projectile_engagement(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    optional_keys: Option<Res<Input<KeyCode>>>,
    mut crosshairs_query: Query<&mut Visibility, With<Crosshairs>>,
    planet_query: Query<&Transform, With<Collider>>,
    rapier_context: Res<RapierContext>,
    craft: Query<&Transform, With<Spacecraft>>,
) {
    for pov in craft.iter() {
        let ray_origin = pov.translation;
        let ray_direction = -1.0 * pov.local_z();
        let intersection = rapier_context.cast_ray(
            ray_origin,
            ray_direction,
            150.0, // what's reasonable here...?
            true,
            QueryFilter::only_dynamic(),
        );

        if let Some((planet, distance)) = intersection {
            if let Some(ref keys) = optional_keys {
                if keys.just_pressed(KeyCode::F) {
                    let global_impact_site = ray_origin + (ray_direction * distance);
                    let planet_transform = planet_query.get(planet).unwrap();
                    let local_impact_site = planet_transform.translation - global_impact_site;
                    let radius = 0.15;
                    commands
                        .spawn_bundle(PbrBundle {
                            mesh: meshes.add(Mesh::from(shape::Icosphere {
                                radius,
                                ..Default::default()
                            })),
                            material: materials.add(Color::WHITE.into()),
                            transform: Transform::from_translation(ray_origin),
                            ..Default::default()
                        })
                        .insert(BallisticProjectileTarget {
                            planet,
                            local_impact_site,
                        })
                        .insert(RigidBody::Dynamic)
                        .insert(Collider::ball(radius))
                        .insert(ActiveEvents::COLLISION_EVENTS)
                        .insert(Sensor);
                }
            }
            for mut crosshairs in crosshairs_query.iter_mut() {
                crosshairs.is_visible = true;
            }
        } else {
            for mut crosshairs in crosshairs_query.iter_mut() {
                crosshairs.is_visible = false;
            }
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy)]
enum AppState {
    Startup,
    Playing,
    Paused,
    Menu,
}

fn handle_game_state(mut app_state: ResMut<State<AppState>>, keys: Res<Input<KeyCode>>) {
    use AppState::*;
    use KeyCode::*;
    let next_state =
        keys.get_just_pressed()
            .fold(None, |_state, key| match (*app_state.current(), *key) {
                (Playing, Space) => Some(Paused),
                (Paused, Space) => Some(Playing),
                (Menu, M) => Some(Playing),
                (_, M) => Some(Menu),
                (Startup, _) => Some(Playing),
                _ => None,
            });
    if let Some(state) = next_state {
        let _ = app_state.overwrite_set(state);
    }
}

// Take the latitude (poles are [1,-1]) and the longitude (portion around, starting at (0,0,1))
// and return the x, y, z on the unit sphere.
fn latlon_to_cartesian(lat: f32, lon: f32) -> Vec3 {
    let theta = (lat * 2.0 - 1.0).acos(); // latitude. -1 & 1 are poles. 0 is equator.
    let phi = lon * TAU; // portion around the planet `[0,1)` (from Greenwich)
    let x = theta.sin() * phi.cos();
    let y = theta.sin() * phi.sin();
    let z = theta.cos();
    Vec3::new(x, y, z)
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut rapier_config: ResMut<RapierConfiguration>,
) {
    rapier_config.gravity = Vec3::ZERO;

    let mut rng = rand::thread_rng();
    let mut rf = || rng.gen::<f32>();
    let pair_count = 40;
    for _ in 0..pair_count {
        let position = latlon_to_cartesian(rf(), rf()) * (rf() * 40.0 + 10.0);
        let velocity = latlon_to_cartesian(rf(), rf()) * Vec3::new(10.0, rf() * 0.1, 10.0);
        let radius = rf() + 2.0;
        for side in [-1.0, 1.0] {
            let color = Color::rgb(rf(), rf(), rf());
            spawn_planet(
                radius,
                position * side,
                velocity * side,
                color,
                &mut commands,
                &mut meshes,
                &mut materials,
            );
        }
    }
}
