use eframe::{egui::{self, FontDefinitions, FontData}, epaint::FontFamily};
use egui::FontFamily::Proportional;
use egui::FontId;
use egui::TextStyle::*;
use crate::view::View;

pub fn configure_fonts(_view: &mut View,ctx: &egui::Context){
 
    let mut fonts = FontDefinitions::default();

    // Install my own font (maybe supporting non-latin characters):
    fonts.font_data.insert("JosefinSans".to_owned(),
       FontData::from_static(include_bytes!("assets/fonts/JosefinSans-Regular.ttf"))); // .ttf and .otf supported
    
    // Put my font first (highest priority):
    fonts.families.get_mut(&FontFamily::Proportional).unwrap()
        .insert(0, "JosefinSans".to_owned());
    
    // Put my font as last fallback for monospace:
    fonts.families.get_mut(&FontFamily::Monospace).unwrap()
        .push("JosefinSans".to_owned());
    
    ctx.set_fonts(fonts);


    let mut style = (*ctx.style()).clone();
    style.text_styles = [
        (Heading, FontId::new(22.0, Proportional)),
        (Body, FontId::new(18.0, Proportional)),
        (Monospace, FontId::new(14.0, Proportional)),
        (Button, FontId::new(14.0, Proportional)),
        (Small, FontId::new(10.0, Proportional)),
    ]
    .into();
    ctx.set_style(style);
}