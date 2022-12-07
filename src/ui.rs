use bevy::prelude::*;
use bevy_egui::egui::*;
use bevy_egui::*;

use crate::{from_nick, new_renet_client, GameConfig};

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
            ui.horizontal(|ui| {
                ui.label("Enter a nickname: ");
                ui.text_edit_singleline(&mut game_config.nick);
            });
            if !game_config.connected {
                ui.horizontal(|ui| {
                    ui.label("Click the button to connect: ");
                    if ui.button("CONNECT NOW").clicked() {
                        commands.insert_resource(new_renet_client(from_nick(&game_config.nick)));
                        game_config.connected = true;
                    }
                });
            }
        });
}
