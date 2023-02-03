pub(crate) use chrono::Duration;
use eframe::egui::{self};
use crate::view::init_view::*;
use crate::controller::reformat_dates::format_date_string;

pub fn render_date(view: &mut View, ui: &mut eframe::egui::Ui) {
    let date = format_date_string(view);
    ui.vertical_centered(|ui|{
        ui.add_space(50.0);
        ui.horizontal(|ui|{
            ui.add_space(50.0);
            ui.label(egui::RichText::new(date).heading().color(egui::Color32::from_rgb(255, 255, 255)));
        });
    });
}

pub fn render_date_backdrop(view: &View, ui: &mut eframe::egui::Ui) {
    ui.vertical_centered(|ui|{
        ui.add_space(12.0);
        view.date_backdrop.show_size(ui, egui::vec2(351.0, 199.0));
    });
}

pub fn render_date_arrow(view: &mut View, ui: &mut eframe::egui::Ui, ctx: &egui::Context) {
    let prev = egui::ImageButton::new(view.arrow_left_icon.texture_id(ctx), egui::vec2(7.0, 12.0));
    let next = egui::ImageButton::new(view.arrow_right_icon.texture_id(ctx), egui::vec2(7.0, 12.0));

    ui.vertical_centered(|ui|{
        ui.add_space(50.0);
        ui.horizontal(|ui|{
            ui.add_space(300.0);
            if ui.add(prev.frame(false)).clicked(){
                view.date = view.date - Duration::days(1);
            };
            ui.add_space(20.0);
            if ui.add(next.frame(false)).clicked(){
                view.date = view.date + Duration::days(1);
            };
        });
    });
}