#![windows_subsystem = "windows"]
fn main() -> eframe::Result {
    let viewport = eframe::egui::ViewportBuilder::default()
                       .with_inner_size(app::WINDOW_INIT_SIZE)
        .with_min_inner_size(app::WINDOW_MIN_SIZE)
        .with_icon(eframe::icon_data::from_png_bytes(include_bytes!("../img/logo.png")).unwrap());
    let options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };

    eframe::run_native(app::NAME, options, Box::new(|context| {
        egui_extras::install_image_loaders(&context.egui_ctx);
        Ok(Box::new(app::App::new(context)))
    }))
}


mod app;
mod engine;
mod paint_object;
mod color_selector;
mod egui_painter;
mod floating_window;
