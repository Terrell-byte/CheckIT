use eframe::egui::{self};
use crate::view::init_view::*;

pub fn render_taskbar(view: &mut View, ui: &mut eframe::egui::Ui) {
    ui.vertical_centered(|ui|{
        ui.add_space(612.78);
        view.taskbar_background.show_size(ui, egui::vec2(386.0, 72.38));
    });
}
pub fn render_taskbar_elements(view: &mut View, ui: &mut eframe::egui::Ui, ctx: &egui::Context) {
    let home_icon = egui::ImageButton::new(view.home_icon.texture_id(ctx), egui::vec2(47.0, 47.0));
    let add_task_icon = egui::ImageButton::new(view.add_task.texture_id(ctx), egui::vec2(51.0, 51.0));
    let profile_icon = egui::ImageButton::new(view.profile_icon.texture_id(ctx), egui::vec2(35.0, 35.0));
    ui.vertical_centered(|ui| {
        ui.add_space(626.04);
        ui.horizontal(|ui| {
            ui.add_space(53.08);
            if ui.add(home_icon.frame(false)).clicked() {
                todo!("Go to home page");
            };
            ui.add_space(53.08);
            if ui.add(add_task_icon.frame(false)).clicked() {
                todo!("Go to add task page");
            };
            ui.add_space(53.08);
            if ui.add(profile_icon.frame(false)).clicked() {
                todo!("Go to profile page");
            };
        });
    });
}