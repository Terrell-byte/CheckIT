use eframe::egui;
use egui_extras::RetainedImage;
use chrono::{Datelike, Local, DateTime};
use crate::date::{render_date, render_date_backdrop, render_date_arrow};

pub struct InitView {
    pub date_backdrop: RetainedImage,
    pub arrow_left_icon: RetainedImage,
    pub arrow_right_icon: RetainedImage,
    pub date: DateTime<Local>,
}

impl InitView {
    pub fn new() -> Self {
        //load images
        let date_backdrop = RetainedImage::from_image_bytes("assets/date_backdrop.png", include_bytes!("assets/date_backdrop.png")).unwrap();
        let arrow_left_icon = RetainedImage::from_image_bytes("assets/arrow_left.png", include_bytes!("assets/arrow_left.png")).unwrap();
        let arrow_right_icon = RetainedImage::from_image_bytes("assets/arrow_right.png", include_bytes!("assets/arrow_right.png")).unwrap();

        let date = Local::now();
        Self { date_backdrop, date, arrow_left_icon, arrow_right_icon}
    }

    pub fn format_date_string(&self) -> String {

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

}

impl eframe::App for InitView {
    fn update(mut self: &mut InitView, ctx: &egui::Context, _frame: &mut eframe::Frame) {
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
            render_date_backdrop(&mut self,ui);
        });


        //date layers
        egui::CentralPanel::default()
        .frame(frame)
        .show(ctx, |ui| {
            //load UI elements
            render_date(&mut self,ui);
        });
        egui::CentralPanel::default()
        .frame(frame)
        .show(ctx, |ui| {
            //load UI elements
            render_date_arrow(&mut self, ui, ctx);
        });
    }
}
