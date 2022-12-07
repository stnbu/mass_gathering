use bevy::prelude::*;
use bevy_egui::egui::*;
use bevy_egui::*;
use egui_extras::{Size, StripBuilder, TableBuilder};

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
        .show(ctx.ctx_mut(), |ui| {
            let button_text = match game_config.menu_page {
                0 => "Choose a Nickname...",
                1 => "Next...",
                _ => "",
            };
            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                if !button_text.is_empty() && ui.button(button_text).clicked() {
                    game_config.menu_page += 1;
                }
            });
        });

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

    match game_config.menu_page {
        0 => {
            CentralPanel::default()
                .frame(Frame {
                    ..Default::default()
                })
                .show(ctx.ctx_mut(), |ui| {
                    StripBuilder::new(ui)
                        .size(Size::exact(30.0))
                        .size(Size::remainder())
                        .vertical(|mut strip| {
                            strip.cell(|ui| {
                                ui.label(RichText::new("Input Bindings:"));
                            });
                            strip.cell(|ui| {
                                build_table(ui);
                            });
                        });
                });
        }
        1 => {
            CentralPanel::default()
                .frame(Frame {
                    ..Default::default()
                })
                .show(ctx.ctx_mut(), |ui| {
                    StripBuilder::new(ui)
                        .size(Size::exact(65.0))
                        .size(Size::exact(30.0))
                        .size(Size::remainder())
                        .vertical(|mut strip| {
                            strip.cell(|ui| {
                                ui.label(RichText::new("Choose an eight-character nickname."));
                            });
                            strip.cell(|ui| {
                                ui.horizontal(|ui| {
                                    ui.label("Nickname: ");
                                    ui.text_edit_singleline(&mut game_config.nick);
                                });
                            });
                        });
                });
        }
        2 => {
            CentralPanel::default()
                .frame(Frame {
                    ..Default::default()
                })
                .show(ctx.ctx_mut(), |ui| {
                    StripBuilder::new(ui)
                        .size(Size::exact(65.0))
                        .size(Size::remainder())
                        .vertical(|mut strip| {
                            strip.cell(|ui| {
                                ui.label(RichText::new("Click to connect."));
                            });
                            strip.cell(|ui| {
                                if !game_config.connected && ui.button("CONNECT NOW").clicked() {
                                    warn!("clicked!!");
                                    commands.insert_resource(new_renet_client(from_nick(
                                        &game_config.nick,
                                    )));
                                    game_config.connected = true;
                                }
                            });
                        });
                });
        }
        _ => panic!(),
    }
}

fn build_table(ui: &mut egui::Ui) {
    TableBuilder::new(ui)
        .striped(false)
        .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
        .column(Size::initial(120.0))
        .column(Size::initial(270.0))
        .column(Size::remainder())
        .resizable(false)
        .header(20.0, |mut header| {
            header.col(|ui| {
                ui.label(RichText::new("Key"));
            });
            header.col(|ui| {
                ui.label(RichText::new("Mouse"));
            });
            header.col(|ui| {
                ui.label(RichText::new("Function"));
            });
        })
        .body(|mut body| {
            let row_height = 10.0;
            body.row(row_height, |mut row| {
                row.col(|ui| {
                    ui.label(RichText::new(""));
                });
                row.col(|ui| {
                    ui.label(RichText::new("Up / Down"));
                });
                row.col(|ui| {
                    ui.label(RichText::new("Pitch up / Pitch down"));
                });
            });
            body.row(row_height, |mut row| {
                row.col(|ui| {
                    ui.label(RichText::new(""));
                });
                row.col(|ui| {
                    ui.label(RichText::new("Left / Right"));
                });
                row.col(|ui| {
                    ui.label(RichText::new("Yaw left / Yaw right"));
                });
            });
            body.row(row_height, |mut row| {
                row.col(|ui| {
                    ui.label(RichText::new(""));
                });
                row.col(|ui| {
                    ui.label(RichText::new("Scroll Wheel Up/Down"));
                });
                row.col(|ui| {
                    ui.label(RichText::new("Roll left / Roll right"));
                });
            });
        });
}
