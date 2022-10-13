use bevy::{
    prelude::*,
    render::camera::{RenderTarget, Viewport},
};
use bevy_rapier3d::prelude::{ActiveEvents, Collider, RigidBody, Sensor};
use std::f32::consts::TAU;

pub struct SpaceCamera;

impl Plugin for SpaceCamera {
    fn build(&self, app: &mut App) {
        app.init_resource::<CameraConfig>()
            .add_startup_system(spawn_camera);
    }
}

pub struct CameraConfig {
    pub transform: Transform,
}

impl Default for CameraConfig {
    fn default() -> Self {
        Self {
            transform: Transform::default(),
        }
    }
}

#[derive(Debug, Default, Component)]
pub struct Movement {
    axis_gain: Vec3,
    pub speed: f32,
}

fn _get_primary_window_size(windows: &Res<Windows>) -> Vec2 {
    let window = windows.get_primary().unwrap();
    Vec2::new(window.width() as f32, window.height() as f32)
}
use bevy::window::WindowId;

fn spawn_camera(mut commands: Commands, config: Res<CameraConfig>) {
    commands
        .spawn_bundle(Camera3dBundle {
            transform: config.transform,
            ..Default::default()
        })
        .insert(Movement::default())
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(1.0))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Sensor);
}

fn _spawn_broken_stereo_camera(mut commands: Commands, config: Res<CameraConfig>) {
    commands.spawn_bundle(Camera3dBundle {
        camera: Camera {
            viewport: Some(Viewport {
                physical_size: UVec2 { x: 600, y: 600 },
                physical_position: UVec2 { x: 0, y: 0 },
                ..Default::default()
            }),
            target: RenderTarget::Window(WindowId::primary()),
            priority: 0,
            ..Default::default()
        },
        transform: config.transform,
        ..Default::default()
    });
    commands.spawn_bundle(Camera3dBundle {
        camera: Camera {
            viewport: Some(Viewport {
                physical_size: UVec2 { x: 600, y: 600 },
                physical_position: UVec2 { x: 600, y: 0 },
                ..Default::default()
            }),
            target: RenderTarget::Window(WindowId::primary()),
            priority: 1,
            ..Default::default()
        },
        transform: config.transform,
        ..Default::default()
    });
}

pub fn move_forward(
    mut camera_query: Query<(&mut Transform, &Movement), With<Camera>>,
    time: Res<Time>,
) {
    for (mut transform, movement) in camera_query.iter_mut() {
        let direction = transform.local_z();
        transform.translation -= direction * time.delta_seconds() * movement.speed;
    }
}

pub fn steer(
    keys: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Movement), With<Camera>>,
) {
    let gain = 0.2;
    let nudge = TAU / 10000.0;
    let mut roll = 0.0;
    let mut pitch = 0.0;
    let mut yaw = 0.0;
    let mut had_input = false;

    let (mut transform, mut movement) = query.get_single_mut().unwrap();

    for key in keys.get_pressed() {
        match key {
            KeyCode::Left => {
                roll += nudge * (movement.axis_gain.z + 1.0);
                had_input = true;
                movement.axis_gain.z += gain;
            }
            KeyCode::Right => {
                roll -= nudge * (movement.axis_gain.z + 1.0);
                had_input = true;
                movement.axis_gain.z += gain;
            }
            KeyCode::Up => {
                pitch += nudge * (movement.axis_gain.x + 1.0);
                had_input = true;
                movement.axis_gain.x += gain;
            }
            KeyCode::Down => {
                pitch -= nudge * (movement.axis_gain.x + 1.0);
                had_input = true;
                movement.axis_gain.x += gain;
            }
            KeyCode::Z => {
                yaw += nudge * (movement.axis_gain.y + 1.0);
                had_input = true;
                movement.axis_gain.y += gain;
            }
            KeyCode::X => {
                yaw -= nudge * (movement.axis_gain.y + 1.0);
                had_input = true;
                movement.axis_gain.y += gain;
            }
            KeyCode::PageUp => {
                movement.speed += 0.5;
            }
            KeyCode::PageDown => {
                movement.speed -= 0.5;
            }
            _ => (),
        }
    }
    if !had_input {
        if movement.axis_gain.x > 0.0 {
            movement.axis_gain.x -= gain;
            if movement.axis_gain.x < 0.0 {
                movement.axis_gain.x = 0.0;
            }
        }
        if movement.axis_gain.y > 0.0 {
            movement.axis_gain.y -= gain;
            if movement.axis_gain.y < 0.0 {
                movement.axis_gain.y = 0.0;
            }
        }
        if movement.axis_gain.z > 0.0 {
            movement.axis_gain.z -= gain;
            if movement.axis_gain.z < 0.0 {
                movement.axis_gain.z = 0.0;
            }
        }
    }

    if roll != 0.0 || pitch != 0.0 || yaw != 0.0 {
        let local_x = transform.local_x();
        let local_y = transform.local_y();
        let local_z = transform.local_z();
        transform.rotate(Quat::from_axis_angle(local_x, pitch));
        transform.rotate(Quat::from_axis_angle(local_z, roll));
        transform.rotate(Quat::from_axis_angle(local_y, yaw));
    }
}
