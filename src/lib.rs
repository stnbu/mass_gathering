use bevy::app::PluginGroupBuilder;

use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_rapier3d::prelude::{NoUserData, RapierConfiguration, RapierPhysicsPlugin};
use rand::Rng;
use std::f32::consts::{PI, TAU};

mod craft;
mod helpscreen;
mod physics;
pub mod prelude;

use prelude::*;

pub struct FullGame;

impl PluginGroup for FullGame {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(Core)
            .add(SpacecraftPlugin)
            .add(Spacetime)
    }
}

pub struct SpacecraftPlugin;

impl Plugin for SpacecraftPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SpacecraftConfig>()
            .add_event::<ProjectileCollisionEvent>()
            .add_event::<HotPlanetEvent>()
            .add_event::<FireProjectileEvent>()
            .add_systems(
                Update,
                (
                    move_forward,
                    control,
                    stars,
                    signal_hot_planet,
                    fire_on_hot_planet,
                    animate_projectile_explosion,
                    handle_hot_planet,
                    set_ar_default_visibility.before(handle_hot_planet),
                    move_projectiles.before(handle_despawn_planet),
                    transfer_projectile_momentum,
                    spawn_projectile_explosion_animation.after(handle_despawn_planet),
                    handle_projectile_despawn.after(spawn_projectile_explosion_animation),
                )
                    .run_if(in_state(AppState::Playing)),
            )
            .add_systems(Startup, spacecraft_setup)
            .add_systems(Update, helpscreen.run_if(in_state(AppState::Help)));
    }
}

pub struct Spacetime;

impl Plugin for Spacetime {
    fn build(&self, app: &mut App) {
        app.init_resource::<PhysicsConfig>()
            .add_event::<DeltaEvent>()
            .add_event::<PlanetCollisionEvent>()
            .add_event::<DespawnPlanetEvent>()
            .add_systems(
                Update,
                (
                    handle_despawn_planet,
                    signal_freefall_delta.before(handle_despawn_planet),
                    handle_freefall.before(handle_despawn_planet),
                    handle_planet_collisions.before(handle_despawn_planet),
                    transfer_planet_momentum.before(handle_despawn_planet),
                )
                    .run_if(in_state(AppState::Playing)),
            );
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy, Default, States)]
enum AppState {
    #[default]
    Playing,
    Help,
}

pub struct Core;

impl Plugin for Core {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(bevy::log::LogPlugin {
                      filter: "info,wgpu_core=warn,wgpu_hal=off,mass_gathering=debug,mass_gathering::networking=debug".into(),
                            level: bevy::log::Level::DEBUG,
                       }));
        app.add_systems(Update, bevy::window::close_on_esc);
        app.add_plugins(EguiPlugin)
            .add_state::<AppState>()
            .add_systems(Startup, disable_rapier_gravity)
            .add_systems(Update, timer_despawn)
            .add_plugins(RapierPhysicsPlugin::<NoUserData>::default());
    }
}

fn disable_rapier_gravity(mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = Vec3::ZERO;
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

pub(crate) fn radius_to_mass(radius: f32) -> f32 {
    (4.0 / 3.0) * PI * radius.powf(3.0)
}

pub(crate) fn mass_to_radius(mass: f32) -> f32 {
    ((mass * (3.0 / 4.0)) / PI).powf(1.0 / 3.0)
}

#[derive(Component)]
pub struct Star;

#[derive(Component)]
pub struct DespawnTimer {
    pub ttl: Timer,
}

pub fn timer_despawn(
    mut commands: Commands,
    mut despawn_query: Query<(Entity, &mut DespawnTimer)>,
    time: Res<Time>,
) {
    for (entity, mut despawn_timer) in despawn_query.iter_mut() {
        despawn_timer.ttl.tick(time.delta());
        if despawn_timer.ttl.finished() {
            debug!("Despawning by timer: {entity:?}");
            commands.entity(entity).despawn();
        }
    }
}

pub fn my_planets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut rng = rand::thread_rng();
    let mut rf = || rng.gen::<f32>();
    let pair_count = 18;
    for _ in 0..pair_count {
        let position = latlon_to_cartesian(rf(), rf()) * (rf() * 40.0 + 10.0);
        let velocity = latlon_to_cartesian(rf(), rf()) * Vec3::new(10.0, rf() * 0.1, 10.0) * 0.1;
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

    // poorly implemented stars!!
    let star_count = 40;
    for _ in 0..star_count {
        let position = latlon_to_cartesian(rf(), rf()) * 400.0;
        let radius = rf() * 0.3 + 0.7;
        let (r, w, y) = (rf() * 40.0, rf() * 400.0, rf() * 20.0);
        let star_colored = (Color::RED * r + Color::WHITE * w + Color::YELLOW * y) * 1000.0;
        commands
            .spawn(PbrBundle {
                mesh: meshes.add(
                    Mesh::try_from(shape::Icosphere {
                        radius,
                        ..default()
                    })
                    .unwrap(),
                ),
                material: materials.add(star_colored.into()),
                transform: Transform::from_translation(position),
                ..default()
            })
            .insert(Star);
    }
}

#[derive(Default)]
pub struct Prev(pub Vec3);

fn stars(
    mut stars_query: Query<&mut Transform, (With<Star>, Without<Spacecraft>)>,

    spacecraft_query: Query<&mut Transform, With<Spacecraft>>,
    mut previous: Local<Prev>,
) {
    let spacecraft = spacecraft_query.get_single().unwrap();
    for mut star in stars_query.iter_mut() {
        star.translation += spacecraft.translation - previous.0;
    }
    previous.0 = spacecraft.translation;
}
