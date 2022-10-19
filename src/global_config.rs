use bevy::prelude::{
    shape, Camera, Component, PointLight, Query, Res, ResMut, Transform, Vec3, With, Without,
};
use bevy_egui::{
    egui::{style::Margin, Color32, Frame, RichText, SidePanel, Slider},
    EguiContext,
};

pub fn global_config_gui(mut ctx: ResMut<EguiContext>, mut global_config: ResMut<GlobalConfig>) {
    SidePanel::right("global_config")
        .frame(Frame {
            outer_margin: Margin::symmetric(10.0, 20.0),
            fill: Color32::TRANSPARENT,
            ..Default::default()
        })
        .show(ctx.ctx_mut(), |ui| {
            for (index, light) in global_config.lights.iter_mut().enumerate() {
                ui.label(RichText::new(format!("Light #{}", index)).color(Color32::GREEN));
                ui.add(Slider::new(&mut light.brightness, 0.0..=1280000.0).text("brightness"));
                ui.add(Slider::new(&mut light.position.x, -200.0..=200.0).text("x"));
                ui.add(Slider::new(&mut light.position.y, -200.0..=200.0).text("y"));
                ui.add(Slider::new(&mut light.position.z, -200.0..=200.0).text("z"));
                ui.separator();
            }
        });
}

use bevy::prelude::{Assets, Color, Commands, Mesh, PbrBundle, StandardMaterial};

#[derive(Component, Default)]
pub struct GlobalConfigSubscriber;

// pub fn on_global_config_changes(
//     global_config: Res<GlobalConfig>,
//     mut query: Query<
//         (
//             &mut rt::RelativeTransform,
//             Option<(&mut PointLight, &LightIndex)>,
//         ),
//         With<GlobalConfigSubscriber>,
//     >,
//     camera_query: Query<&Transform, (With<SingletonCamera>, Without<GlobalConfigSubscriber>)>,
// ) {
//     if global_config.is_changed() {
//         for (mut transform, light_opt) in query.iter_mut() {
//             if let Some((mut light, index)) = light_opt {
//                 if let Ok(camera) = camera_query.get_single() {
//                     if let Some(config) = global_config.lights.get(index.0) {
//                         transform.transform.translation = (*config).position + camera.translation;
//                         light.intensity = (*config).brightness;
//                     }
//                 }
//             }
//         }
//     }
// }

#[derive(Component)]
pub struct LightIndex(pub usize);

pub struct LightConfig {
    pub position: Vec3,
    pub brightness: f32,
}

pub struct GlobalConfig {
    pub lights: Vec<LightConfig>,
}

impl Default for GlobalConfig {
    fn default() -> Self {
        Self {
            lights: (0..5)
                .map(|_| LightConfig {
                    position: Vec3::ZERO,
                    brightness: 0.0,
                })
                .collect(),
        }
    }
}
