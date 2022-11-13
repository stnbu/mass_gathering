use bevy::prelude::{App, ClearColor, Color, Transform, Vec3};
use bevy::window::WindowPlugin;
use bevy::{DefaultPlugins, MinimalPlugins};
use mass_gathering::prelude::{my_planets, PhysicsConfig, SpacecraftConfig};
use mass_gathering::FullGame;

/*
        group.add(bevy_core::CorePlugin::default());
        group.add(bevy_time::TimePlugin::default());
        group.add(bevy_app::ScheduleRunnerPlugin::default());
*/

/*
.add_plugin(bevy::log::LogPlugin)
.add_plugin(bevy::transform::TransformPlugin)
.add_plugin(bevy::hierarchy::HierarchyPlugin)
.add_plugin(bevy::diagnostic::DiagnosticsPlugin)
.add_plugin(bevy::input::InputPlugin)
.add_plugin(bevy::window::WindowPlugin)
.add_plugin(bevy::asset::AssetPlugin)
.add_plugin(bevy::scene::ScenePlugin)
.add_plugin(bevy::winit::WinitPlugin)
.add_plugin(bevy::render::RenderPlugin)
.add_plugin(bevy::gilrs::GilrsPlugin)
*/

/*
[features]
default = [
  "animation",
  "bevy_asset",
  "bevy_audio",
  "bevy_gilrs",
  "bevy_scene",
  "bevy_winit",
  "render",
  "png",
  "hdr",
  "vorbis",
  "x11",
  "filesystem_watcher",
]
 */

use bevy::pbr::PbrPlugin;

fn main() {
    let d = 60.0 / 3.0_f32.powf(0.5); // about right for my_planets
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugin(bevy::log::LogPlugin)
        .add_plugin(bevy::transform::TransformPlugin)
        .add_plugin(bevy::hierarchy::HierarchyPlugin)
        .add_plugin(bevy::diagnostic::DiagnosticsPlugin)
        .add_plugin(bevy::input::InputPlugin)
        .add_plugin(bevy::window::WindowPlugin)
        .add_plugin(bevy::asset::AssetPlugin)
        .add_plugin(bevy::scene::ScenePlugin)
        .add_plugin(bevy::winit::WinitPlugin)
        .add_plugin(bevy::render::RenderPlugin)
        .add_plugin(bevy::core_pipeline::CorePipelinePlugin)
        .add_plugin(bevy::pbr::PbrPlugin)
        .add_plugin(bevy::gilrs::GilrsPlugin)
        .insert_resource(ClearColor(Color::MIDNIGHT_BLUE * 0.1))
        .insert_resource(PhysicsConfig {
            sims_per_frame: 1,
            trails: true,
            trail_ttl: 2500 * 5,
        })
        .insert_resource(SpacecraftConfig {
            stereo_enabled: false,
            start_transform: Transform::from_xyz(d, d, d).looking_at(Vec3::ZERO, Vec3::Y),
            impact_magnitude: 100.0,
            ..Default::default()
        })
        .add_plugins(FullGame)
        .add_startup_system(my_planets)
        .run();
}

// lesigh orig
/*
        group.add(bevy_core::CorePlugin::default());
        group.add(bevy_time::TimePlugin::default());
        group.add(bevy_app::ScheduleRunnerPlugin::default());
*/

/*
        group.add(bevy_log::LogPlugin::default());
        group.add(bevy_core::CorePlugin::default());
        group.add(bevy_time::TimePlugin::default());
        group.add(bevy_transform::TransformPlugin::default());
        group.add(bevy_hierarchy::HierarchyPlugin::default());
        group.add(bevy_diagnostic::DiagnosticsPlugin::default());
        group.add(bevy_input::InputPlugin::default());
        group.add(bevy_window::WindowPlugin::default());

        #[cfg(feature = "bevy_asset")]
        group.add(bevy_asset::AssetPlugin::default());

        #[cfg(feature = "debug_asset_server")]
        group.add(bevy_asset::debug_asset_server::DebugAssetServerPlugin::default());

        #[cfg(feature = "bevy_scene")]
        group.add(bevy_scene::ScenePlugin::default());

        #[cfg(feature = "bevy_winit")]
        group.add(bevy_winit::WinitPlugin::default());

        #[cfg(feature = "bevy_render")]
        group.add(bevy_render::RenderPlugin::default());

        #[cfg(feature = "bevy_core_pipeline")]
        group.add(bevy_core_pipeline::CorePipelinePlugin::default());

        #[cfg(feature = "bevy_sprite")]
        group.add(bevy_sprite::SpritePlugin::default());

        #[cfg(feature = "bevy_text")]
        group.add(bevy_text::TextPlugin::default());

        #[cfg(feature = "bevy_ui")]
        group.add(bevy_ui::UiPlugin::default());

        #[cfg(feature = "bevy_pbr")]
        group.add(bevy_pbr::PbrPlugin::default());

        // NOTE: Load this after renderer initialization so that it knows about the supported
        // compressed texture formats
        #[cfg(feature = "bevy_gltf")]
        group.add(bevy_gltf::GltfPlugin::default());

        #[cfg(feature = "bevy_audio")]
        group.add(bevy_audio::AudioPlugin::default());

        #[cfg(feature = "bevy_gilrs")]
        group.add(bevy_gilrs::GilrsPlugin::default());

        #[cfg(feature = "bevy_animation")]
        group.add(bevy_animation::AnimationPlugin::default());
*/

/*
[features]
default = [
  "animation",
  "bevy_asset",
  "bevy_audio",
  "bevy_gilrs",
  "bevy_scene",
  "bevy_winit",
  "render",
  "png",
  "hdr",
  "vorbis",
  "x11",
  "filesystem_watcher",
]
*/

// a backup
/*
        group.add(bevy_log::LogPlugin::default());
        group.add(bevy_transform::TransformPlugin::default());
        group.add(bevy_hierarchy::HierarchyPlugin::default());
        group.add(bevy_diagnostic::DiagnosticsPlugin::default());
        group.add(bevy_input::InputPlugin::default());
        group.add(bevy_window::WindowPlugin::default());

        group.add(bevy_asset::AssetPlugin::default());
        group.add(bevy_scene::ScenePlugin::default());
        group.add(bevy_winit::WinitPlugin::default());
        group.add(bevy_render::RenderPlugin::default());
        group.add(bevy_gilrs::GilrsPlugin::default());
*/

/*
bevy::log::LogPlugin
bevy::transform::TransformPlugin
bevy::hierarchy::HierarchyPlugin
bevy::diagnostic::DiagnosticsPlugin
bevy::input::InputPlugin
bevy::window::WindowPlugin
bevy::asset::AssetPlugin
bevy::scene::ScenePlugin
bevy::winit::WinitPlugin
bevy::render::RenderPlugin
bevy::gilrs::GilrsPlugin
*/
