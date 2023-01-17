use eframe::egui;
use egui_extras::RetainedImage;

struct InitView;

impl eframe::App for InitView {


    
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        
        //load image from file so it can be used
        let date_backdrop = RetainedImage::from_image_bytes(
            "date_backdrop.png",
            include_bytes!("date_backdrop.png"),
        ).unwrap();
        
        //background color
        let frame = egui::containers::Frame {
            fill: egui::Color32::from_rgb(241, 233, 218),
            ..Default::default()
        };
        
        //main window
        egui::CentralPanel::default()
        .frame(frame)
        .show(ctx, |ui| {
            //load image
            date_backdrop.show(ui);
        
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
        Box::new(|_cc| Box::new(InitView)),
    )
}