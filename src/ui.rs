use bevy::prelude::*;
use bevy_egui::egui::FontFamily::Monospace;
use bevy_egui::egui::*;
use bevy_egui::*;
use egui_extras::{Size, StripBuilder, TableBuilder};

use crate::GameConfig;

const FILL_COLOR: Color32 = Color32::from_rgba_premultiplied(0, 0, 0, 240);
pub fn menu_frame(mut ctx: ResMut<EguiContext>) {
    TopBottomPanel::top("top_panel")
        .resizable(false)
        .min_height(200.0)
        .frame(Frame {
            fill: FILL_COLOR,
            ..Default::default()
        })
        .show(ctx.ctx_mut(), |_| ());

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
