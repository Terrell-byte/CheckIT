use eframe::{run_native, epi::App, egui, NativeOptions};
use image::GenericImageView;
struct Tasks{
    texture: Option<(egui::Vec2, egui::TextureId)>,
}

impl App for Tasks {
    fn name(&self) -> &str {
        "CheckIt"
    }

    fn update(&mut self,ctx: &eframe::egui::CtxRef,frame: &mut eframe::epi::Frame<'_>) {
        //background color
        let frame = egui::containers::Frame {
            fill: egui::Color32::from_rgb(241, 233, 218),
            ..Default::default()
        };
    
        if self.texture.is_none() {
            // Load the image:
            let image_data = include_bytes!("date_backdrop.png");
            let image = image::load_from_memory(image_data).expect("Failed to load image");
            let image_buffer = image.to_rgba8();
            let size = (image.width() as usize, image.height() as usize);
            let pixels = image_buffer.into_vec();
            assert_eq!(size.0 * size.1 * 4, pixels.len());
            let pixels: Vec<_> = pixels
                .chunks_exact(4)
                .map(|p| egui::Color32::from_rgba_unmultiplied(p[0], p[1], p[2], p[3]))
                .collect();
    
            // Allocate a texture:
            let texture = ctx.tex_allocator().alloc_srgba_premultiplied(size, &pixels);
            let size = egui::Vec2::new(size.0 as f32, size.1 as f32);
            self.texture = Some((size, texture));
        }
    
        //main window
        egui::CentralPanel::default().frame(frame).show(ctx, |ui| {
            if let Some((size, texture)) = self.texture {
                ui.image(texture, size);
            }
        });
    }
}

fn main(){
    let app: Tasks = Tasks{texture: None};
    let win_options = eframe::NativeOptions{
        initial_window_size: Some(egui::Vec2::new(386.0, 636.0)),
        always_on_top: true,
        resizable: false,
        ..Default::default()
    };
    run_native(Box::new(app), win_options);
}