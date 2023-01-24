use eframe::egui::{self};
use init_view::View;

mod init_view;
mod render_date;
mod font_loader;
mod reformat_dates;
mod task;
mod render_taskbar;

fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(386.0, 685.0)),
        resizable: false,
        ..Default::default()
    };
    let _view = init_view::View::new();
        eframe::run_native(
        "CheckIt",
        options,
        Box::new(|_cc| Box::new(View::new())),
    )   
}