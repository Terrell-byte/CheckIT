use eframe::egui::{self};
use init_view::InitView;

mod init_view;
mod date;
mod font_loader;
mod reformat_dates;

fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(386.0, 685.0)),
        resizable: false,
        ..Default::default()
    };
    let _view = init_view::InitView::new();
        eframe::run_native(
        "CheckIt",
        options,
        Box::new(|_cc| Box::new(InitView::new())),
    )
    
}