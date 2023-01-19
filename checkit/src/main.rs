use eframe::egui::{self};
use egui_extras::RetainedImage;
use chrono::{Datelike, Local, DateTime, Duration};

struct InitView {
    date_backdrop: RetainedImage,
    date: DateTime<Local>,
}

impl InitView {
    fn new() -> Self {
        //load images
        let date_backdrop = RetainedImage::from_image_bytes(
            "date_backdrop.png",
            include_bytes!("date_backdrop.png"),
        ).unwrap();
        let date = Local::now();
        Self { date_backdrop, date,}
    }

    fn format_date_string(&self) -> String {

        if self.date.year() == Local::now().year() && self.date.month() == Local::now().month() && self.date.day() == Local::now().day() {
            return "Today".to_string();
        }

        let day_of_week = match self.date.weekday() {
            chrono::Weekday::Mon => "Monday",
            chrono::Weekday::Tue => "Tuesday",
            chrono::Weekday::Wed => "Wednesday",
            chrono::Weekday::Thu => "Thursday",
            chrono::Weekday::Fri => "Friday",
            chrono::Weekday::Sat => "Saturday",
            chrono::Weekday::Sun => "Sunday",
        };
    
        let month = match self.date.month() {
            1 => "Jan",
            2 => "Feb",
            3 => "Mar",
            4 => "Apr",
            5 => "May",
            6 => "Jun",
            7 => "Jul",
            8 => "Aug",
            9 => "Sep",
            10 => "Oct",
            11 => "Nov",
            12 => "Dec",
            _ => "",
        };
    
        format!("{} {}, {}", day_of_week, self.date.day(), month)
    }


    fn render_date_backdrop(&self, ui: &mut eframe::egui::Ui) {
        ui.vertical_centered(|ui|{
            ui.add_space(12.0);
            self.date_backdrop.show_size(ui, egui::vec2(351.0, 199.0));
        });
    }

    fn render_date(&mut self, ui: &mut eframe::egui::Ui) {
        let date = self.format_date_string();
        ui.vertical_centered(|ui|{
            ui.add_space(30.0);
            ui.horizontal(|ui|{
                ui.add_space(50.0);
                ui.colored_label(egui::Color32::from_rgb(255, 255, 255), date);
                ui.add_space(30.0);
                if ui.button("prev").clicked() {
                    self.date = self.date - Duration::days(1);
                }
                if ui.button("next").clicked() {
                    self.date = self.date + Duration::days(1);
                }
            });
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
        resizable: false,
        ..Default::default()
    };

    eframe::run_native(
        "CheckIt",
        options,
        Box::new(|_cc| Box::new(InitView::new())),
    )
}
