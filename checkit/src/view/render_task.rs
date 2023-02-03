use eframe::{egui::{self}};
use crate::view::init_view::*;

pub fn render_task(view: &mut View,ui: &mut eframe::egui::Ui, ctx: &egui::Context, frame: egui::Frame) {
    egui::CentralPanel::default()
    .frame(frame)
    .show(ctx, |ui| { 
        ui.vertical_centered(|ui| {
            ui.add_space(260.0);
            view.task_background.show_size(ui, egui::vec2(319.66, 59.11));

        });
    });
    
}