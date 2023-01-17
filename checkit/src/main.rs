use eframe::egui::{self};
use egui_extras::RetainedImage;

struct InitView {
    date_backdrop: RetainedImage,
}

impl InitView {
    fn new() -> Self {

        //load images
        let date_backdrop = RetainedImage::from_image_bytes(
            "date_backdrop.png",
            include_bytes!("date_backdrop.png"),
        ).unwrap();
        Self { date_backdrop }
    }
    fn render_date_backdrop(&self, ui: &mut eframe::egui::Ui) {
        ui.vertical_centered(|ui|{
            ui.add_space(12.0);
            self.date_backdrop.show_size(ui, egui::vec2(351.0, 199.0));
        });

    }
    fn render_date(&self, ui: &mut eframe::egui::Ui) {
        let date = "Error loading date";
        ui.vertical_centered(|ui|{
            ui.add_space(12.0);
            ui.colored_label(egui::Color32::from_rgb(255, 255, 255), date);
        });

    }
}

impl eframe::App for InitView {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        //background color
        let background_color = egui::containers::Frame {
            fill: egui::Color32::from_rgb(241, 233, 218),
            ..Default::default()
        };
        let frame = egui::containers::Frame {
            fill: egui::Color32::from_rgba_premultiplied(0, 0, 0, 0),
            ..Default::default()
        };

        //background layer
        egui::CentralPanel::default()
        .frame(background_color)
        .show(ctx, |ui| {
            //load background
            self.render_date_backdrop(ui);
        });
        //layer 1
        egui::CentralPanel::default()
        .frame(frame)
        .show(ctx, |ui| {
            //load UI elements
            self.render_date(ui);
        });
    }
}

fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(386.0, 636.0)),
        always_on_top: true,
        resizable: false,
        ..Default::default()
    };

    eframe::run_native(
        "CheckIt",
        options,
        Box::new(|_cc| Box::new(InitView::new())),
    )
}
