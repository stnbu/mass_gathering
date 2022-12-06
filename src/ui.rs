use bevy::prelude::*;
use bevy_egui::egui::FontFamily::Monospace;
use bevy_egui::egui::*;
use bevy_egui::*;
use egui_extras::{Size, StripBuilder, TableBuilder};

use crate::GameConfig;

const FILL_COLOR: Color32 = Color32::from_rgba_premultiplied(0, 0, 0, 240);
pub fn menu_frame(mut ctx: ResMut<EguiContext>, mut game_config: ResMut<GameConfig>) {
    TopBottomPanel::top("top_panel")
        .resizable(false)
        .min_height(200.0)
        .frame(Frame {
            fill: FILL_COLOR,
            ..Default::default()
        })
        .show(ctx.ctx_mut(), |ui| {
            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                if ui.button("NEXT>").clicked() {
                    game_config.menu_page += 1;
                }
            });
        });

    SidePanel::left("left_panel")
        .resizable(false)
        .min_width(300.0)
        .frame(Frame {
            fill: FILL_COLOR,
            ..Default::default()
        })
        .show(ctx.ctx_mut(), |_| ());

    SidePanel::right("right_panel")
        .resizable(false)
        .min_width(100.0)
        .frame(Frame {
            fill: FILL_COLOR,
            ..Default::default()
        })
        .show(ctx.ctx_mut(), |_| ());

    TopBottomPanel::bottom("bottom_panel")
        .resizable(false)
        .min_height(0.0)
        .frame(Frame {
            fill: FILL_COLOR,
            ..Default::default()
        })
        .show(ctx.ctx_mut(), |_| ());

    match game_config.menu_page {
        0 => {
            CentralPanel::default()
                .frame(Frame {
                    fill: FILL_COLOR,
                    ..Default::default()
                })
                .show(ctx.ctx_mut(), |ui| {
                    StripBuilder::new(ui)
                        .size(Size::exact(65.0))
                        .size(Size::exact(30.0))
                        .size(Size::remainder())
                        .vertical(|mut strip| {
                            strip.cell(|ui| {
                                styled_text_label(50.0, ui, "CLICK ANYWHERE TO BEGIN!");
                            });
                            strip.cell(|ui| {
                                styled_text_label(22.0, ui, ".. Input Bindings ..");
                            });
                            strip.cell(|ui| {
                                build_table(ui);
                            });
                        });
                });
        }
        _ => {
            CentralPanel::default()
                .frame(Frame {
                    fill: FILL_COLOR,
                    ..Default::default()
                })
                .show(ctx.ctx_mut(), |ui| {
                    StripBuilder::new(ui)
                        .size(Size::exact(65.0))
                        .size(Size::exact(30.0))
                        .size(Size::remainder())
                        .vertical(|mut strip| {
                            strip.cell(|ui| {
                                styled_text_label(22.0, ui, "Choose an eight-character nickname.");
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
    }
}

fn styled_text_label(height: f32, ui: &mut egui::Ui, text: &str) {
    ui.label(RichText::new(text).color(Color32::GREEN).font(FontId {
        size: height,
        family: Monospace,
    }));
}

fn build_table(ui: &mut egui::Ui) {
    TableBuilder::new(ui)
        .striped(false)
        .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
        .column(Size::initial(120.0))
        .column(Size::initial(270.0))
        .column(Size::remainder())
        .resizable(false)
        .header(50.0, |mut header| {
            header.col(|ui| {
                styled_text_label(22.0, ui, "Key");
            });
            header.col(|ui| {
                styled_text_label(22.0, ui, "Mouse");
            });
            header.col(|ui| {
                styled_text_label(20.0, ui, "Function");
            });
        })
        .body(|mut body| {
            let row_height = 22.0;
            body.row(row_height, |mut row| {
                row.col(|ui| {
                    styled_text_label(18.0, ui, "");
                });
                row.col(|ui| {
                    styled_text_label(18.0, ui, "Up / Down");
                });
                row.col(|ui| {
                    styled_text_label(18.0, ui, "Pitch up / Pitch down");
                });
            });
            body.row(row_height, |mut row| {
                row.col(|ui| {
                    styled_text_label(18.0, ui, "");
                });
                row.col(|ui| {
                    styled_text_label(18.0, ui, "Left / Right");
                });
                row.col(|ui| {
                    styled_text_label(18.0, ui, "Yaw left / Yaw right");
                });
            });
            body.row(row_height, |mut row| {
                row.col(|ui| {
                    styled_text_label(18.0, ui, "");
                });
                row.col(|ui| {
                    styled_text_label(18.0, ui, "Scroll Wheel Up/Down");
                });
                row.col(|ui| {
                    styled_text_label(18.0, ui, "Roll left / Roll right");
                });
            });
        });
}

// // //
// It tries to cobble together "connect logic"
// // //
pub fn draw_lobby_list(ui: &mut Ui, lobby_list: Vec<LobbyListing>) -> Option<(u64, bool)> {
    ui.separator();
    ui.heading("Lobby list");
    if lobby_list.is_empty() {
        ui.label("No lobbies available");
        return None;
    }

    let mut connect_server_id = None;
    TableBuilder::new(ui)
        .striped(true)
        .cell_layout(Layout::left_to_right(Align::Min))
        .column(Size::exact(12.))
        .column(Size::remainder())
        .column(Size::exact(40.))
        .column(Size::exact(60.))
        .header(12.0, |mut header| {
            header.col(|_| {});
            header.col(|ui| {
                ui.label("Name");
            });
        })
        .body(|mut body| {
            for lobby in lobby_list.iter() {
                body.row(30., |mut row| {
                    row.col(|ui| {
                        if lobby.is_protected {
                            ui.label("🔒");
                        }
                    });

                    row.col(|ui| {
                        ui.label(&lobby.name);
                    });

                    row.col(|ui| {
                        ui.label(format!("{}/{}", lobby.current_clients, lobby.max_clients));
                    });

                    row.col(|ui| {
                        if ui.button("connect").clicked() {
                            connect_server_id = Some((lobby.id, lobby.is_protected));
                        }
                    });
                });
            }
        });

    connect_server_id
}

pub fn draw_main_screen(
    ui_state: &mut UiState,
    state: &mut AppState,
    lobby_list: Vec<LobbyListing>,
    ctx: &egui::Context,
) {
    egui::CentralPanel::default().show(ctx, |ui| {
        egui::Area::new("buttons")
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
            .show(ui.ctx(), |ui| {
                ui.set_width(300.);
                ui.set_height(300.);
                ui.vertical_centered(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("Nick:");
                        ui.text_edit_singleline(&mut ui_state.username)
                    });

                    ui.horizontal(|ui| {
                        ui.label("Lobby name:");
                        ui.text_edit_singleline(&mut ui_state.lobby_name)
                    });

                    ui.horizontal(|ui| {
                        ui.label("Lobby password:")
                            .on_hover_text("Password can be empty");
                        egui::TextEdit::singleline(&mut ui_state.password)
                            .password(true)
                            .ui(ui)
                    });

                    ui.vertical_centered_justified(|ui| {
                        if ui.button("Host").clicked() {
                            if ui_state.username.is_empty() || ui_state.lobby_name.is_empty() {
                                ui_state.error =
                                    Some("Nick or Lobby name can't be empty".to_owned());
                            } else {
                                let server = ChatServer::new(
                                    ui_state.lobby_name.clone(),
                                    ui_state.username.clone(),
                                    ui_state.password.clone(),
                                );
                                *state = AppState::HostChat {
                                    chat_server: Box::new(server),
                                };
                            }
                        }
                    });

                    if let Some(error) = &ui_state.error {
                        ui.separator();
                        ui.colored_label(Color32::RED, format!("Error: {}", error));
                    }

                    if let Some((connect_server_id, is_protected)) = draw_lobby_list(ui, lobby_list)
                    {
                        if ui_state.username.is_empty() {
                            ui_state.error = Some("Nick can't be empty".to_owned());
                        } else if is_protected && ui_state.password.is_empty() {
                            ui_state.error =
                                Some("Lobby is protected, please insert a password".to_owned());
                        } else {
                            let (sender, receiver) = mpsc::channel();
                            let password = if is_protected {
                                Some(ui_state.password.clone())
                            } else {
                                None
                            };
                            let request_connection = RequestConnection {
                                username: ui_state.username.clone(),
                                password,
                            };

                            std::thread::spawn(move || {
                                if let Err(e) = connect_token_request(
                                    connect_server_id,
                                    request_connection,
                                    sender,
                                ) {
                                    log::error!(
                                        "Failed to get connect token for server {}: {}",
                                        connect_server_id,
                                        e
                                    );
                                }
                            });

                            *state = AppState::RequestingToken { token: receiver };
                        }
                    }
                });
            });
    });
}
