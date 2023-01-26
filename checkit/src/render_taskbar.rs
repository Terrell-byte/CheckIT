use eframe::egui::{self};
use crate::init_view::View;

pub fn render_taskbar(view: &mut View, ctx: &egui::Context) {
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
pub fn render_home(view: &mut View, ui: &mut eframe::egui::Ui, ctx: &egui::Context) {
    let home_icon = egui::ImageButton::new(view.home_icon.texture_id(ctx), egui::vec2(47.0, 47.0));

    ui.vertical_centered(|ui|{
        ui.add_space(626.04);
        ui.horizontal(|ui|{
            ui.add_space(53.08);
            if ui.add(home_icon.frame(false)).clicked(){
              todo!("Go to home page");
            };
        });
    });
}