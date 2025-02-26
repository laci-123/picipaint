#![windows_subsystem = "windows"]
fn main() -> eframe::Result {
    let viewport = eframe::egui::ViewportBuilder::default()
                       .with_inner_size(app::WINDOW_INIT_SIZE)
                       .with_min_inner_size(app::WINDOW_MIN_SIZE);
    let options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };

    eframe::run_native(app::NAME, options, Box::new(|context| Ok(Box::new(app::App::new(context)))))
}


mod app;
mod paint_object;
mod tool;
mod color_selector;
mod view_transform;
