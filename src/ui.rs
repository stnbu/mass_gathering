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
const TEXT_COLOR: Color32 = Color32::from_rgba_premultiplied(0, 255, 0, 100);

pub fn client_menu_screen(
    mut ctx: ResMut<EguiContext>,
    mut game_config: ResMut<GameConfig>,
    mut commands: Commands,
) {
    TopBottomPanel::top("top_panel")
        .resizable(false)
        .min_height(120.0)
        .frame(Frame::default())
        .show(ctx.ctx_mut(), |_| ());

    SidePanel::left("left_panel")
        .resizable(false)
        .min_width(180.0)
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

    let greeting =
        RichText::new("
Wonder Woman's origin story (from Golden to Bronze Age) relates that she was sculpted from clay by her mother Queen Hippolyta and was given a life as an Amazon, along with superhuman powers as gifts by the Greek gods.

In 2011, DC changed her background with the retcon that she is the biological daughter of Zeus and Hippolyta, jointly raised by her mother and her aunts Antiope and Menalippe. The character has changed in depiction over the decades, including briefly losing her powers entirely in the late 1960s; by the 1980s, artist George Perez gave her an athletic look and emphasized her Amazonian heritage.

She possesses an arsenal of magical items, including the Lasso of Truth, a pair of indestructible bracelets, a tiara which serves as a projectile, and, in older stories, a range of devices based on Amazon technology.

. . .

If two or more connected users have select \"Autostart\", the game will begin immediately.

If at least one client has not chosen autostart, the game will wait until all of its player slots have been filled.

Enter a nickname between one and eight characters, choose whether you prefer autostart then click the button to connect and play.


")
            .color(TEXT_COLOR)
            .font(FontId {
                size: 15.0,
                family: Monospace,
            });

    CentralPanel::default()
        .frame(Frame {
            ..Default::default()
        })
        .show(ctx.ctx_mut(), |ui| {
            ui.label(
                RichText::new("The Mass Gathering")
                    .color(TEXT_COLOR)
                    .font(FontId {
                        size: 30.0,
                        family: Monospace,
                    }),
            );
            ui.label(greeting);
            ui.label(
                RichText::new("Enter a nickname between 1 and 8 charaters then click the button.")
                    .color(TEXT_COLOR)
                    .font(FontId {
                        size: 20.0,
                        family: Monospace,
                    }),
            );
            ui.separator();
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new("Enter a nickname: ")
                        .color(TEXT_COLOR)
                        .font(FontId {
                            size: 20.0,
                            family: Monospace,
                        }),
                );
                ui.text_edit_singleline(&mut game_config.nick);
            });
            ui.horizontal(|ui| {
                ui.label(RichText::new("Autostart: ").color(TEXT_COLOR).font(FontId {
                    size: 20.0,
                    family: Monospace,
                }));
                ui.checkbox(&mut game_config.autostart, "");
            });
            if !game_config.connected {
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new("Click the button to connect: ")
                            .color(TEXT_COLOR)
                            .font(FontId {
                                size: 20.0,
                                family: Monospace,
                            }),
                    );
                    let autostart = game_config.autostart;
                    if ui.button("Connect Now!").clicked() {
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
