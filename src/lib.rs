use bevy::app::PluginGroupBuilder;
use bevy::input::mouse::MouseButtonInput;
use bevy::prelude::*;
use bevy_rapier3d::prelude::{NoUserData, RapierConfiguration, RapierPhysicsPlugin};
use std::f32::consts::PI;
mod physics;
use physics::*;

mod craft;
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
            .add_startup_system(spacecraft_setup)
            .add_event::<ProjectileCollisionEvent>()
            .add_event::<HotPlanetEvent>()
            .add_event::<FireProjectileEvent>()
            .add_system_set(
                SystemSet::on_update(AppState::Playing)
                    .with_system(control)
                    .with_system(signal_hot_planet)
                    .with_system(fire_on_hot_planet)
                    .with_system(animate_projectile_explosion)
                    .with_system(handle_hot_planet)
                    .with_system(set_ar_default_visibility.before(handle_hot_planet))
                    .with_system(move_projectiles.before(handle_despawn_planet))
                    .with_system(transfer_projectile_momentum)
                    // FIXME: even though `handle_despawn_planet` added by another plugin?
                    .with_system(spawn_projectile_explosion_animation.after(handle_despawn_planet))
                    .with_system(
                        handle_projectile_despawn.after(spawn_projectile_explosion_animation),
                    ),
            );
    }
}

pub fn let_light(mut commands: Commands) {
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        ..default()
    });
}

pub struct Spacetime;

impl Plugin for Spacetime {
    fn build(&self, app: &mut App) {
        app.init_resource::<PhysicsConfig>()
            .add_event::<DeltaEvent>()
            .add_event::<PlanetCollisionEvent>()
            .add_event::<DespawnPlanetEvent>()
            .add_system_set(
                SystemSet::on_update(AppState::Playing)
                    .with_system(handle_despawn_planet)
                    .with_system(signal_freefall_delta.before(handle_despawn_planet))
                    .with_system(handle_freefall.before(handle_despawn_planet))
                    .with_system(handle_planet_collisions.before(handle_despawn_planet))
                    .with_system(transfer_planet_momentum.before(handle_despawn_planet)),
            );
    }
}

pub struct Core;

impl Plugin for Core {
    fn build(&self, app: &mut App) {
        #[cfg(debug_assertions)]
        {
            debug!("DEBUG LEVEL LOGGING ! !");
            app.add_plugins(DefaultPlugins.set(bevy::log::LogPlugin {
                filter: "warn,mass_gathering=debug".into(),
                level: bevy::log::Level::DEBUG,
            }));
        }

        #[cfg(not(debug_assertions))]
        {
            error!("We have no logging, and yet you SEE this message...?");
            // FIXME: num-triangles on a mesh is a different thing
            app.insert_resource(Msaa { samples: 4 });
            app.add_plugins(DefaultPlugins);
        }

        app.add_startup_system(let_light);
        app.add_system(bevy::window::close_on_esc);

        app.add_state(AppState::Paused)
            .add_startup_system(disable_rapier_gravity)
            .add_system(handle_game_state)
            .add_system(timer_despawn)
            .add_plugin(RapierPhysicsPlugin::<NoUserData>::default());
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy)]
enum AppState {
    Playing,
    Paused,
}

fn disable_rapier_gravity(mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = Vec3::ZERO;
}

fn handle_game_state(
    mut app_state: ResMut<State<AppState>>,
    keys: Res<Input<KeyCode>>,
    mouse_button_input_events: EventReader<MouseButtonInput>,
    mut windows: ResMut<Windows>,
) {
    use AppState::*;
    use KeyCode::*;
    let next_state = if *app_state.current() == Paused && !mouse_button_input_events.is_empty() {
        let window = windows.get_primary_mut().unwrap();
        window.set_cursor_visibility(false);
        Some(Playing)
    } else {
        keys.get_just_pressed()
            .fold(None, |_state, key| match (*app_state.current(), *key) {
                (Playing, P | H | M) => {
                    let window = windows.get_primary_mut().unwrap();
                    window.set_cursor_visibility(true);
                    Some(Paused)
                }
                (_, _) => Some(Playing),
            })
    };
    if let Some(state) = next_state {
        let _ = app_state.overwrite_set(state);
    }
}

pub fn radius_to_mass(radius: f32) -> f32 {
    (4.0 / 3.0) * PI * radius.powf(3.0)
}

pub(crate) fn mass_to_radius(mass: f32) -> f32 {
    ((mass * (3.0 / 4.0)) / PI).powf(1.0 / 3.0)
}

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

// // // // // // // // // // // // // // // // // // // //
// // // // // // // // // // // // // // // // // // // //
// // // // // // // // // // // // // // // // // // // //
// Verbatim copy of src/lib.rs from prototype

use std::time::Duration;

//use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_renet::renet::{
    ChannelConfig, ReliableChannelConfig, RenetConnectionConfig, UnreliableChannelConfig,
    NETCODE_KEY_BYTES,
};
use serde::{Deserialize, Serialize};

pub const PRIVATE_KEY: &[u8; NETCODE_KEY_BYTES] = b"an example very very secret key."; // 32-bytes
pub const PROTOCOL_ID: u64 = 11;
pub const SERVER_ADDR: &str = "192.168.1.43";
pub const PORT_NUMBER: u16 = 5241;

#[derive(Debug, Component)]
pub struct Player {
    pub id: u64,
}

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, Component, Resource)]
pub struct PlayerInput {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
}

#[derive(Debug, Serialize, Deserialize, Component)]
pub enum PlayerCommand {
    BasicAttack { cast_at: Vec3 },
}

pub enum ClientChannel {
    Input,
    Command,
}

pub enum ServerChannel {
    ServerMessages,
    NetworkedEntities,
}

#[derive(Debug, Serialize, Deserialize, Component)]
pub enum ServerMessages {
    PlayerCreate {
        entity: Entity,
        id: u64,
        translation: [f32; 3],
    },
    PlayerRemove {
        id: u64,
    },
    SpawnProjectile {
        entity: Entity,
        translation: [f32; 3],
    },
    DespawnProjectile {
        entity: Entity,
    },
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NetworkedEntities {
    pub entities: Vec<Entity>,
    pub translations: Vec<[f32; 3]>,
}

impl From<ClientChannel> for u8 {
    fn from(channel_id: ClientChannel) -> Self {
        match channel_id {
            ClientChannel::Command => 0,
            ClientChannel::Input => 1,
        }
    }
}

impl ClientChannel {
    pub fn channels_config() -> Vec<ChannelConfig> {
        vec![
            ReliableChannelConfig {
                channel_id: Self::Input.into(),
                message_resend_time: Duration::ZERO,
                ..Default::default()
            }
            .into(),
            ReliableChannelConfig {
                channel_id: Self::Command.into(),
                message_resend_time: Duration::ZERO,
                ..Default::default()
            }
            .into(),
        ]
    }
}

impl From<ServerChannel> for u8 {
    fn from(channel_id: ServerChannel) -> Self {
        match channel_id {
            ServerChannel::NetworkedEntities => 0,
            ServerChannel::ServerMessages => 1,
        }
    }
}

impl ServerChannel {
    pub fn channels_config() -> Vec<ChannelConfig> {
        vec![
            UnreliableChannelConfig {
                channel_id: Self::NetworkedEntities.into(),
                sequenced: true, // We don't care about old positions
                ..Default::default()
            }
            .into(),
            ReliableChannelConfig {
                channel_id: Self::ServerMessages.into(),
                message_resend_time: Duration::from_millis(200),
                ..Default::default()
            }
            .into(),
        ]
    }
}

pub fn client_connection_config() -> RenetConnectionConfig {
    RenetConnectionConfig {
        send_channels_config: ClientChannel::channels_config(),
        receive_channels_config: ServerChannel::channels_config(),
        ..Default::default()
    }
}

pub fn server_connection_config() -> RenetConnectionConfig {
    RenetConnectionConfig {
        send_channels_config: ServerChannel::channels_config(),
        receive_channels_config: ClientChannel::channels_config(),
        ..Default::default()
    }
}

/// set up a simple 3D scene
pub fn setup_level(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(10., 1., 10.))),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            transform: Transform::from_xyz(0.0, -1.0, 0.0),
            ..Default::default()
        })
        .insert(Collider::cuboid(5., 0.5, 5.));
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
}

#[derive(Debug, Component)]
pub struct Projectile {
    pub duration: Timer,
}

pub fn spawn_fireball(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    translation: Vec3,
    mut direction: Vec3,
) -> Entity {
    if !direction.is_normalized() {
        direction = Vec3::X;
    }
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                radius: 0.1,
                subdivisions: 5,
            })),
            material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
            transform: Transform::from_translation(translation),
            ..Default::default()
        })
        .insert(RigidBody::Dynamic)
        .insert(LockedAxes::ROTATION_LOCKED | LockedAxes::TRANSLATION_LOCKED_Y)
        .insert(Collider::ball(0.1))
        .insert(Velocity::linear(direction * 10.))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Projectile {
            duration: Timer::from_seconds(1.5, TimerMode::Once),
        })
        .id()
}

/// A 3D ray, with an origin and direction. The direction is guaranteed to be normalized.
#[derive(Debug, PartialEq, Copy, Clone, Default)]
pub struct Ray3d {
    pub(crate) origin: Vec3,
    pub(crate) direction: Vec3,
}

impl Ray3d {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Ray3d { origin, direction }
    }

    pub fn from_screenspace(
        windows: &Res<Windows>,
        camera: &Camera,
        camera_transform: &GlobalTransform,
    ) -> Option<Self> {
        let window = windows.get_primary().unwrap();
        let cursor_position = match window.cursor_position() {
            Some(c) => c,
            None => return None,
        };

        let view = camera_transform.compute_matrix();
        let screen_size = camera.logical_target_size()?;
        let projection = camera.projection_matrix();
        let far_ndc = projection.project_point3(Vec3::NEG_Z).z;
        let near_ndc = projection.project_point3(Vec3::Z).z;
        let cursor_ndc = (cursor_position / screen_size) * 2.0 - Vec2::ONE;
        let ndc_to_world: Mat4 = view * projection.inverse();
        let near = ndc_to_world.project_point3(cursor_ndc.extend(near_ndc));
        let far = ndc_to_world.project_point3(cursor_ndc.extend(far_ndc));
        let ray_direction = far - near;

        Some(Ray3d::new(near, ray_direction))
    }

    pub fn intersect_y_plane(&self, y_offset: f32) -> Option<Vec3> {
        let plane_normal = Vec3::Y;
        let plane_origin = Vec3::new(0.0, y_offset, 0.0);
        let denominator = self.direction.dot(plane_normal);
        if denominator.abs() > f32::EPSILON {
            let point_to_point = plane_origin * y_offset - self.origin;
            let intersect_dist = plane_normal.dot(point_to_point) / denominator;
            let intersect_position = self.direction * intersect_dist + self.origin;
            Some(intersect_position)
        } else {
            None
        }
    }
}

// End from prototype
// // // // // // // // // // // // // // // // // // // //
