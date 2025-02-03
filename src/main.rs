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

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(&self.label);
            egui::Frame::canvas(ui.style()).show(ui, |ui| {
                let size = ui.available_size();
                let (mut response, painter) = ui.allocate_painter(size, egui::Sense::drag());
                painter.circle_filled(egui::Pos2 { x: 0.1, y: 0.1 }, 10.0, egui::ecolor::Color32::GREEN);

                if let Some(pointer_pos) = response.interact_pointer_pos() {
                    painter.circle_filled(pointer_pos, 10.0, egui::ecolor::Color32::GREEN);
                    response.mark_changed();
                }
            });
        });
    }
}


// mod paint_object;
