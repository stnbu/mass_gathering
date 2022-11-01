use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_rapier3d::prelude::{NoUserData, RapierConfiguration, RapierPhysicsPlugin};
use rand::Rng;
use std::f32::consts::TAU;

mod physics;
use physics::*;

mod craft;
use craft::*;

mod lines;

pub mod prelude;

pub struct Core;

pub struct FullGame;

impl PluginGroup for FullGame {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(Core).add(SpacecraftPlugin).add(Spacetime);
    }
}

pub struct SpacecraftPlugin;

impl Plugin for SpacecraftPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SpacecraftConfig>()
            .add_system_set(
                SystemSet::on_update(AppState::Playing)
                    .with_system(move_forward)
                    .with_system(steer)
                    .with_system(handle_projectile_engagement)
                    .with_system(handle_projectile_flight)
                    .with_system(animate_projectile_explosion)
                    .with_system(set_planet_markup_default_visibility)
                    .with_system(set_crosshairs_default_visibility)
                    .with_system(
                        handle_hot_planet
                            .after(set_planet_markup_default_visibility)
                            .after(set_crosshairs_default_visibility),
                    ),
            )
            .add_startup_system(spacecraft_setup)
            .add_system(hud)
            .add_system(set_camera_viewports);
    }
}

pub struct Spacetime;

impl Plugin for Spacetime {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(AppState::Playing)
                .with_system(freefall)
                .with_system(collision_events),
        );
    }
}

impl Plugin for Core {
    fn build(&self, app: &mut App) {
        #[cfg(target_arch = "wasm32")]
        app.add_system(handle_browser_resize);
        app.insert_resource(ClearColor(Color::MIDNIGHT_BLUE * 0.1))
            .add_plugins(DefaultPlugins)
            .add_plugin(EguiPlugin)
            .add_state(AppState::Startup)
            .add_system(bevy::window::close_on_esc)
            .add_system(handle_game_state)
            .add_plugin(RapierPhysicsPlugin::<NoUserData>::default());
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

pub fn my_planets(
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

#[cfg(target_arch = "wasm32")]
fn handle_browser_resize(mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    let wasm_window = web_sys::window().unwrap();
    let (target_width, target_height) = (
        wasm_window.inner_width().unwrap().as_f64().unwrap() as f32,
        wasm_window.inner_height().unwrap().as_f64().unwrap() as f32,
    );

    if window.width() != target_width || window.height() != target_height {
        window.set_resolution(target_width, target_height);
    }
}
