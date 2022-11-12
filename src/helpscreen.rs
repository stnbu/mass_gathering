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
                .column(Size::initial(300.0).at_least(200.0))
                .column(Size::remainder().at_least(60.0))
                .resizable(false)
                .header(30.0, |mut header| {
                    header.col(|ui| {
                        ui.heading("Key");
                    });
                    header.col(|ui| {
                        ui.heading("Function");
                    });
                })
                .body(|mut body| {
                    let row_height = 18.0;
                    body.row(row_height, |mut row| {
                        row.col(|ui| {
                            ui.label("Space");
                        });
                        row.col(|ui| {
                            ui.label("Fire Projectile");
                        });
                    });
                    body.row(row_height, |mut row| {
                        row.col(|ui| {
                            ui.label("W / S");
                        });
                        row.col(|ui| {
                            ui.label("Pitch up / Pitch down");
                        });
                    });
                    body.row(row_height, |mut row| {
                        row.col(|ui| {
                            ui.label("A / D");
                        });
                        row.col(|ui| {
                            ui.label("Yaw left / Yaw right");
                        });
                    });
                    body.row(row_height, |mut row| {
                        row.col(|ui| {
                            ui.label("Z / X");
                        });
                        row.col(|ui| {
                            ui.label("Roll left / Roll right");
                        });
                    });

                    body.row(row_height, |mut row| {
                        row.col(|ui| {
                            ui.label("H or P");
                        });
                        row.col(|ui| {
                            ui.label("[P]ause and show this [H]elp screen");
                        });
                    });
                });
        });
}
