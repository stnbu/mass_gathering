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
pub fn draw_lobby_list(ui: &mut Ui) -> Option<String> {
    let mut connect_server_id = None;
    if ui.button("connect").clicked() {
        connect_server_id = Some("chat.net.host.foo:1234".to_string());
    }
    connect_server_id
}

pub fn draw_main_screen(
    // ui_state: &mut UiState,
    // state: &mut AppState,
    // lobby_list: Vec<LobbyListing>,
    ctx: &egui::Context,
) {
    if let Some(address) = draw_lobby_list(ui, lobby_list) {
        let (sender, receiver) = mpsc::channel();
        let request_connection = RequestConnection {
            username: ui_state.username.clone(),
        };

        std::thread::spawn(move || {
            if let Err(e) = connect_token_request(address) {
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

pub fn connect_token_request(
    server_id: u64,
    request_connection: RequestConnection,
    sender: Sender<reqwest::Result<ConnectToken>>,
) -> Result<(), Box<dyn Error>> {
    let client = reqwest::blocking::Client::new();
    let res = client
        .post(format!("http://localhost:8889/server/{server_id}/connect"))
        .json(&request_connection)
        .send()?;
    if let Err(e) = res.error_for_status_ref() {
        sender.send(Err(e))?;
    } else {
        let bytes = res.bytes()?;
        let token = ConnectToken::read(&mut bytes.as_ref())?;
        sender.send(Ok(token))?;
    }

    Ok(())
}
