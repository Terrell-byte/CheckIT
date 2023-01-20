use eframe::egui::{self};
use view::View;

mod view;
mod date;
mod font_loader;
mod reformat_dates;

fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(386.0, 685.0)),
        resizable: false,
        ..Default::default()
    };
    let _view = view::View::new();
        eframe::run_native(
        "CheckIt",
        options,
        Box::new(|_cc| Box::new(View::new())),
    )
    
}