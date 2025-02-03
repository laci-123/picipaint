fn main() -> eframe::Result {
    let viewport = eframe::egui::ViewportBuilder::default()
                       .with_inner_size(app::WINDOW_INIT_SIZE)
                       .with_min_inner_size(app::WINDOW_MIN_SIZE);
    let options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };

    eframe::run_native(app::NAME, options, Box::new(|context| Ok(Box::new(app::App::new(context, "árvíztűrő tükörfúrógép")))))
}


mod app;
