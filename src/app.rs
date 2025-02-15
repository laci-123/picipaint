use eframe::egui::{self, Vec2};
use crate::paint_object::{straight_line::*, PaintObject, PaintObjectMaker};


pub const WINDOW_INIT_SIZE: Vec2 = Vec2::new(800.0, 450.0);
pub const WINDOW_MIN_SIZE:  Vec2 = Vec2::new(300.0, 200.0);
pub const NAME: &'static str     = "PiciPaint";

pub struct App {
    straight_line_maker: StraightLineMaker,
    objects: Vec<Box<dyn PaintObject>>,
}

impl App {
    pub fn new(_context: &eframe::CreationContext) -> Self {
        Self {
            straight_line_maker: StraightLineMaker::new(egui::Stroke::new(3.0, egui::Color32::GREEN)),
            objects: Vec::new(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_pixels_per_point(1.5);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(NAME);
            egui::Frame::canvas(ui.style()).show(ui, |ui| {
                let size = ui.available_size();
                let (response, painter) = ui.allocate_painter(size, egui::Sense::click());
                if let Some(object) = self.straight_line_maker.update(&response) {
                    self.objects.push(Box::new(object));
                }
                self.straight_line_maker.draw(&painter);

                for object in self.objects.iter() {
                    object.draw(&painter);
                }
            });
        });
    }
}
