use eframe::egui::{self};
use crate::init_view::View;

pub fn render_taskbar(view: &mut View, ui: &mut eframe::egui::Ui, ctx: &egui::Context) {
    let frame = egui::containers::Frame {
        fill: egui::Color32::from_rgba_premultiplied(0, 0, 0, 0),
        ..Default::default()
    };
    egui::CentralPanel::default()
    .frame(frame)
    .show(ctx, |ui| {
        ui.vertical_centered(|ui|{
            ui.add_space(612.78);
            view.taskbar_background.show_size(ui, egui::vec2(386.0, 72.38));
        });
    });
}