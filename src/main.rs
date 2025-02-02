// use paint_object::*;
// use straight_line::*;
// use freehand_curve::*;
use eframe::egui;


fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
                       .with_inner_size([800.0, 450.0])
                       .with_min_inner_size([300.0, 200.0]),
        ..Default::default()
    };

    eframe::run_native("Próba", options, Box::new(|context| Ok(Box::new(App::new(context, "árvíztűrő tükörfúrógép")))))
}


struct App {
    label: String,
}

impl App {
    fn new(_context: &eframe::CreationContext, label: &str) -> Self {
        Self {
            label: String::from(label),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_pixels_per_point(1.5);
        
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.label(&self.label);
        });
    }
}


// mod paint_object;
