use crate::{networking::*, GameConfig};
use bevy::prelude::*;
use bevy_egui::{
    egui::{
        style::Margin, CentralPanel, Color32, FontFamily::Monospace, FontId, Frame, RichText,
        SidePanel, TextEdit, TopBottomPanel,
    },
    EguiContext,
};

const FRAME_FILL: Color32 = Color32::TRANSPARENT;
const TEXT_COLOR: Color32 = Color32::from_rgba_premultiplied(0, 255, 0, 100);

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
            for (&id, &client_data) in lobby.clients.iter() {
                let nick = to_nick(id);
                let pad = String::from_iter((0..(8 - nick.len())).map(|_| ' '));
                let autostart = if client_data.preferences.autostart {
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
