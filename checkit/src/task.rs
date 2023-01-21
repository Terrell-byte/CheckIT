use eframe::egui::{self};
use crate::view::View;

pub fn render_task(view: &mut View, _ui: &mut eframe::egui::Ui, ctx: &egui::Context) {

    let frame = egui::containers::Frame {
        fill: egui::Color32::from_rgba_premultiplied(0, 0, 0, 0),
        ..Default::default()
    };
    egui::CentralPanel::default()
    .frame(frame)
    .show(ctx, |ui| {
        ui.vertical_centered(|ui|{
            ui.add_space(260.0);

            //create a iterator for the tasks that is loaded from the datebase and then render them, and give them a 18.0 space between them
            //make sure that all of this is in a scroll area
            view.task_background.show_size(ui, egui::vec2(319.66, 59.11));
            ui.add_space(18.0);

        });
    });

}