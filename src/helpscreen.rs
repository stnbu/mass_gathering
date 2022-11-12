use bevy::prelude::ResMut;
use bevy_egui::{
    egui::{
        CentralPanel, Color32, FontFamily::Monospace, FontId, Frame, RichText, SidePanel,
        TopBottomPanel,
    },
    EguiContext,
};
use egui_extras::{Size, TableBuilder};

pub fn helpscreen(mut ctx: ResMut<EguiContext>) {
    let fill_color = Color32::from_rgba_premultiplied(0, 0, 0, 240);
    TopBottomPanel::top("top_panel")
        .resizable(false)
        .min_height(200.0)
        .frame(Frame {
            fill: fill_color,
            ..Default::default()
        })
        .show(ctx.ctx_mut(), |_| ());

    SidePanel::left("left_panel")
        .resizable(false)
        .min_width(300.0)
        .frame(Frame {
            fill: fill_color,
            ..Default::default()
        })
        .show(ctx.ctx_mut(), |_| ());

    SidePanel::right("right_panel")
        .resizable(false)
        .min_width(100.0)
        .frame(Frame {
            fill: fill_color,
            ..Default::default()
        })
        .show(ctx.ctx_mut(), |_| ());

    TopBottomPanel::bottom("bottom_panel")
        .resizable(false)
        .min_height(0.0)
        .frame(Frame {
            fill: fill_color,
            ..Default::default()
        })
        .show(ctx.ctx_mut(), |_| ());

    CentralPanel::default()
        .frame(Frame {
            fill: fill_color,
            ..Default::default()
        })
        .show(ctx.ctx_mut(), |ui| {
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
                            styled_text_label(18.0, ui, "Space");
                        });
                        row.col(|ui| {
                            styled_text_label(18.0, ui, "Left click");
                        });
                        row.col(|ui| {
                            styled_text_label(18.0, ui, "Fire Projectile");
                        });
                    });
                    body.row(row_height, |mut row| {
                        row.col(|ui| {
                            styled_text_label(18.0, ui, "W / S");
                        });
                        row.col(|ui| {
                            styled_text_label(18.0, ui, "Mouse up+down");
                        });
                        row.col(|ui| {
                            styled_text_label(18.0, ui, "Pitch up / Pitch down");
                        });
                    });
                    body.row(row_height, |mut row| {
                        row.col(|ui| {
                            styled_text_label(18.0, ui, "A / D");
                        });
                        row.col(|ui| {
                            styled_text_label(18.0, ui, "Mouse left+right");
                        });
                        row.col(|ui| {
                            styled_text_label(18.0, ui, "Yaw left / Yaw right");
                        });
                    });
                    body.row(row_height, |mut row| {
                        row.col(|ui| {
                            styled_text_label(18.0, ui, "Z / X");
                        });
                        row.col(|_| {});
                        row.col(|ui| {
                            styled_text_label(18.0, ui, "Roll left / Roll right");
                        });
                    });

                    body.row(row_height, |mut row| {
                        row.col(|ui| {
                            styled_text_label(18.0, ui, "PgUp / PgDn");
                        });
                        row.col(|ui| {
                            styled_text_label(18.0, ui, "Mouse wheel forward+backward");
                        });
                        row.col(|ui| {
                            styled_text_label(18.0, ui, "Speed plus / Speed minus");
                        });
                    });

                    body.row(row_height, |mut row| {
                        row.col(|ui| {
                            styled_text_label(18.0, ui, "H or P");
                        });
                        row.col(|_| {});
                        row.col(|ui| {
                            styled_text_label(18.0, ui, "[P]ause and show this [H]elp screen");
                        });
                    });
                });
        });
}

fn styled_text_label(height: f32, ui: &mut egui::Ui, text: &str) {
    ui.label(RichText::new(text).color(Color32::GREEN).font(FontId {
        size: height,
        family: Monospace,
    }));
}
