use eframe::egui::{self, Vec2, ecolor::Color32};


pub const WINDOW_INIT_SIZE: Vec2 = Vec2::new(800.0, 450.0);
pub const WINDOW_MIN_SIZE:  Vec2 = Vec2::new(300.0, 200.0);
pub const NAME: &'static str     = "PiciPaint";

pub struct App {
    label: String,
}

impl App {
    pub fn new(_context: &eframe::CreationContext, label: &str) -> Self {
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
                painter.circle_filled(egui::Pos2 { x: 0.1, y: 0.1 }, 10.0, Color32::GREEN);

                if let Some(pointer_pos) = response.interact_pointer_pos() {
                    painter.circle_filled(pointer_pos, 10.0, Color32::GREEN);
                    response.mark_changed();
                }
            });
        });
    }
}
