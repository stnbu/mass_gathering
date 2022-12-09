use bevy::prelude::*;
use bevy_egui::{
    egui::{
        style::Margin, CentralPanel, Color32, FontFamily::Monospace, FontId, Frame, RichText,
        SidePanel, TopBottomPanel,
    },
    EguiContext,
};

use crate::{networking::*, GameConfig};

const FRAME_FILL: Color32 = Color32::TRANSPARENT;
const TEXT_COLOR: Color32 = Color32::from_rgba_premultiplied(0, 255, 0, 230);

pub fn client_menu_screen(
    mut ctx: ResMut<EguiContext>,
    mut game_config: ResMut<GameConfig>,
    mut commands: Commands,
) {
    TopBottomPanel::top("top_panel")
        .resizable(false)
        .min_height(200.0)
        .frame(Frame::default())
        .show(ctx.ctx_mut(), |_| ());

    SidePanel::left("left_panel")
        .resizable(false)
        .min_width(300.0)
        .frame(Frame::default())
        .show(ctx.ctx_mut(), |_| ());

    SidePanel::right("right_panel")
        .resizable(false)
        .min_width(100.0)
        .frame(Frame::default())
        .show(ctx.ctx_mut(), |_| ());

    TopBottomPanel::bottom("bottom_panel")
        .resizable(false)
        .min_height(0.0)
        .frame(Frame::default())
        .show(ctx.ctx_mut(), |_| ());

    CentralPanel::default()
        .frame(Frame {
            ..Default::default()
        })
        .show(ctx.ctx_mut(), |ui| {
            ui.label("Enter a nickname between 1 and 8 charaters then click the button.");
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Enter a nickname: ");
                ui.text_edit_singleline(&mut game_config.nick);
            });
            ui.horizontal(|ui| {
                ui.label("Autostart: ");
                ui.checkbox(&mut game_config.autostart, "autostart");
            });
            if !game_config.connected {
                ui.horizontal(|ui| {
                    ui.label("Click the button to connect: ");
                    let autostart = game_config.autostart;
                    if ui.button("CONNECT NOW").clicked() {
                        commands.insert_resource(client::new_renet_client(
                            from_nick(&game_config.nick),
                            ClientPreferences { autostart },
                        ));
                        game_config.connected = true;
                    }
                });
            }
        });
}

pub fn client_waiting_screen(mut ctx: ResMut<EguiContext>, lobby: Res<Lobby>) {
    SidePanel::left("client_waiting_screen")
        .resizable(false)
        .min_width(250.0)
        .frame(Frame {
            outer_margin: Margin::symmetric(10.0, 20.0),
            fill: FRAME_FILL,
            ..Default::default()
        })
        .show(ctx.ctx_mut(), |ui| {
            ui.label(
                RichText::new("Waiting for players...")
                    .color(TEXT_COLOR)
                    .font(FontId {
                        size: 20.0,
                        family: Monospace,
                    }),
            );
            ui.separator();
            for (&id, &client_preferences) in lobby.clients.iter() {
                let nick = to_nick(id);
                let pad = String::from_iter((0..(8 - nick.len())).map(|_| ' '));
                let autostart = if client_preferences.autostart {
                    "autostart"
                } else {
                    "wait"
                };
                let text = format!("{nick}{pad}>  {autostart}");
                ui.label(RichText::new(text).color(TEXT_COLOR).font(FontId {
                    size: 16.0,
                    family: Monospace,
                }));
            }
        });
}
