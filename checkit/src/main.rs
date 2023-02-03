use eframe::egui::{self};
use crate::view::init_view::*;

//define crate modules
pub mod model {

}
pub mod view {
    pub mod init_view;
    pub mod render_date;
    pub mod render_task;
    pub mod render_taskbar;
}

pub mod controller {
    pub mod font_loader;
    pub mod reformat_dates;
}
fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(386.0, 685.0)),
        resizable: false,
        ..Default::default()
    };
    let _view = view::init_view::View::new();
        eframe::run_native(
        "CheckIt",
        options,
        Box::new(|_cc| Box::new(View::new())),
    )   
}