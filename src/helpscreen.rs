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
    // .show_inside(ui, |_| ())
    TopBottomPanel::top("top_panel")
        .resizable(false)
        .min_height(200.0)
        .frame(Frame {
            fill: Color32::from_rgba_premultiplied(0, 0, 0, 32 * 7),
            ..Default::default()
        })
        .show(ctx.ctx_mut(), |_| ());

    SidePanel::left("left_panel")
        .resizable(false)
        .min_width(300.0)
        .frame(Frame {
            fill: Color32::from_rgba_premultiplied(0, 0, 0, 32 * 7),
            ..Default::default()
        })
        .show(ctx.ctx_mut(), |_| ());

    SidePanel::right("right_panel")
        .resizable(false)
        .min_width(300.0)
        .frame(Frame {
            fill: Color32::from_rgba_premultiplied(0, 0, 0, 32 * 7),
            ..Default::default()
        })
        .show(ctx.ctx_mut(), |_| ());

    TopBottomPanel::bottom("bottom_panel")
        .resizable(false)
        .min_height(0.0)
        .frame(Frame {
            fill: Color32::from_rgba_premultiplied(0, 0, 0, 32 * 7),
            ..Default::default()
        })
        .show(ctx.ctx_mut(), |_| ());

    CentralPanel::default()
        .frame(Frame {
            fill: Color32::from_rgba_premultiplied(0, 0, 0, 32 * 7),
            ..Default::default()
        })
        .show(ctx.ctx_mut(), |ui| {
            TableBuilder::new(ui)
                .striped(false)
                .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                .column(Size::initial(60.0).at_least(40.0))
                .column(Size::initial(100.0).at_least(70.0))
                .column(Size::remainder().at_least(60.0))
                .resizable(false)
                .header(30.0, |mut header| {
                    header.col(|ui| {
                        ui.heading("Key");
                    });
                    header.col(|ui| {
                        ui.heading("Mouse");
                    });
                    header.col(|ui| {
                        ui.heading("Function");
                    });
                })
                .body(|mut body| {
                    let row_height = 18.0;
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
                        row.col(|ui| {
                            styled_text_label(18.0, ui, "");
                        });
                        row.col(|ui| {
                            styled_text_label(18.0, ui, "Roll left / Roll right");
                        });
                    });

                    body.row(row_height, |mut row| {
                        row.col(|ui| {
                            styled_text_label(18.0, ui, "H or P");
                        });
                        row.col(|ui| {
                            styled_text_label(18.0, ui, "Left click");
                        });
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
