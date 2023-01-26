use eframe::{egui::{self}};
use egui_extras::RetainedImage;
use chrono::{Local, DateTime};
use crate::render_date::{render_date, render_date_backdrop, render_date_arrow};
use crate::font_loader::configure_fonts;
use crate::task::render_task;
use crate::render_taskbar::{render_taskbar, render_home};
pub struct View {
    pub date_backdrop: RetainedImage,
    pub arrow_left_icon: RetainedImage,
    pub arrow_right_icon: RetainedImage,
    pub task_background: RetainedImage,
    pub taskbar_background: RetainedImage,
    pub home_icon: RetainedImage,
    pub profile_icon: RetainedImage,
    pub add_task: RetainedImage,
    pub date: DateTime<Local>,
}

impl View {
    pub fn new() -> Self {
        //load images
        let date_backdrop = RetainedImage::from_image_bytes("assets/date_backdrop.png", include_bytes!("assets/date_backdrop.png")).unwrap();
        let arrow_left_icon = RetainedImage::from_image_bytes("assets/arrow_left.png", include_bytes!("assets/arrow_left.png")).unwrap();
        let arrow_right_icon = RetainedImage::from_image_bytes("assets/arrow_right.png", include_bytes!("assets/arrow_right.png")).unwrap();
        let task_background = RetainedImage::from_image_bytes("assets/task_background.png", include_bytes!("assets/task_background.png")).unwrap();
        let taskbar_background = RetainedImage::from_image_bytes("assets/taskbar_background.png", include_bytes!("assets/taskbar_background.png")).unwrap();
        let home_icon = RetainedImage::from_image_bytes("assets/home_icon.png", include_bytes!("assets/home_icon.png")).unwrap();
        let add_task = RetainedImage::from_image_bytes("assets/add_task.png", include_bytes!("assets/add_task.png")).unwrap();
        let profile_icon = RetainedImage::from_image_bytes("assets/profile_icon.png", include_bytes!("assets/profile_icon.png")).unwrap();

        //load dates
        let date = Local::now();
        
        Self { 
            date_backdrop,
            date,
            arrow_left_icon, 
            arrow_right_icon,
            task_background, 
            taskbar_background, 
            home_icon, 
            profile_icon, 
            add_task
        }
    }
}
impl eframe::App for View {
    fn update(mut self: &mut View, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        //load fonts
        configure_fonts(&mut self,ctx);

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
            render_date_backdrop(&mut self,ui);
        });

        //date layers
        egui::CentralPanel::default()
        .frame(frame)
        .show(ctx, |ui| {
            render_date(&mut self,ui);
        });
        egui::CentralPanel::default()
        .frame(frame)
        .show(ctx, |ui| {
        
            render_date_arrow(&mut self, ui, ctx);
        });

        //task layers
        egui::CentralPanel::default()
        .frame(frame)
        .show(ctx, |ui| {
        
            render_task(&mut self, ui, ctx);
            
        });

        //taskbar layers
        egui::CentralPanel::default()
        .frame(frame)
        .show(ctx, |_ui| {
            
            render_taskbar(&mut self, ctx);
        });
        egui::CentralPanel::default()
        .frame(frame)
        .show(ctx, |ui| {
            ui.horizontal(|ui|{
                render_home(&mut self, ui, ctx)
            });   
        });
    }
}
