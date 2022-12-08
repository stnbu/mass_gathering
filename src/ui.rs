use bevy::prelude::*;
use bevy_egui::{
    egui::{
        style::Margin, CentralPanel, Color32, FontFamily::Monospace, FontId, Frame, RichText,
        SidePanel, TopBottomPanel,
    },
    EguiContext,
};

use crate::{networking::*, GameConfig};

pub fn client_menu(
    mut ctx: ResMut<EguiContext>,
    mut game_config: ResMut<GameConfig>,
    mut commands: Commands,
) {
    TopBottomPanel::top("top_panel")
        .resizable(false)
        .min_height(200.0)
        .frame(Frame {
            ..Default::default()
        })
        .show(ctx.ctx_mut(), |_| ());

    SidePanel::left("left_panel")
        .resizable(false)
        .min_width(300.0)
        .frame(Frame {
            ..Default::default()
        })
        .show(ctx.ctx_mut(), |_| ());

    SidePanel::right("right_panel")
        .resizable(false)
        .min_width(100.0)
        .frame(Frame {
            ..Default::default()
        })
        .show(ctx.ctx_mut(), |_| ());

    TopBottomPanel::bottom("bottom_panel")
        .resizable(false)
        .min_height(0.0)
        .frame(Frame {
            ..Default::default()
        })
        .show(ctx.ctx_mut(), |_| ());

    CentralPanel::default()
        .frame(Frame {
            ..Default::default()
        })
        .show(ctx.ctx_mut(), |ui| {
	    ui.label("Enter a nickname between 1 and 8 charaters then click the button. You _can_ chose \"\" but don't be a troll.");
	    ui.separator();
            ui.horizontal(|ui| {
                ui.label("Enter a nickname: ");
                ui.text_edit_singleline(&mut game_config.nick);
            });
            if !game_config.connected {
                ui.horizontal(|ui| {
                    ui.label("Click the button to connect: ");
                    if ui.button("CONNECT NOW").clicked() {
                        commands.insert_resource(new_renet_client(from_nick(&game_config.nick), ClientPreferences { autostart: true }));
                        game_config.connected = true;
                    }
                });
            }
        });
}

pub fn client_hud(mut ctx: ResMut<EguiContext>) {
    let hud_text = "lorem ipsomething ... text";
    TopBottomPanel::top("hud")
        .frame(Frame {
            outer_margin: Margin::symmetric(10.0, 20.0),
            fill: Color32::TRANSPARENT,
            ..Default::default()
        })
        .show(ctx.ctx_mut(), |ui| {
            ui.label(RichText::new(hud_text).color(Color32::GREEN).font(FontId {
                size: 18.0,
                family: Monospace,
            }));
        });
}
