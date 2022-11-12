use bevy::prelude::ResMut;
use bevy_egui::{
    egui::{
        CentralPanel, Color32, FontFamily::Monospace, FontId, Frame, RichText, SidePanel,
        TopBottomPanel,
    },
    EguiContext,
};

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
            ui.label(
                RichText::new("Hello From Space")
                    .color(Color32::GREEN)
                    .font(FontId {
                        size: 18.0,
                        family: Monospace,
                    }),
            );
        });
}

/*
        egui::TopBottomPanel::top("top_panel")
            .resizable(true)
            .min_height(32.0);

        egui::SidePanel::left("left_panel")
            .resizable(true)
            .default_width(150.0);

        egui::SidePanel::right("right_panel")
            .resizable(true)
            .default_width(150.0);

        egui::TopBottomPanel::bottom("bottom_panel")
            .resizable(false)
            .min_height(0.0);

        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Central Panel");
            });
        });
*/
