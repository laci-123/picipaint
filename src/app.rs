use eframe::egui::{self, Vec2};
use crate::paint_object::{freehand_curve::*, straight_line::*, *};


pub const WINDOW_INIT_SIZE: Vec2 = Vec2::new(800.0, 450.0);
pub const WINDOW_MIN_SIZE:  Vec2 = Vec2::new(300.0, 200.0);
pub const NAME: &'static str     = "PiciPaint";

pub struct App {
    object_makers: Vec<Box<dyn PaintObjectMaker>>,
    objects: Vec<Box<dyn PaintObject>>,
}

impl App {
    pub fn new(_context: &eframe::CreationContext) -> Self {
        Self {
            object_makers: vec![
                Box::new(StraightLineMaker::new(egui::Stroke::new(3.0, egui::Color32::GREEN))),
                Box::new(FreehandCurveMaker::new(egui::Stroke::new(2.0, egui::Color32::BLUE))),
            ],
            objects: vec![],
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
                let (response, painter) = ui.allocate_painter(size, egui::Sense::click_and_drag());

                for object_maker in self.object_makers.iter_mut() {
                    if let Some(object) = object_maker.update(&response) {
                        self.objects.push(object);
                    }
                    object_maker.draw(&painter);
                }

                for object in self.objects.iter() {
                    object.draw(&painter);
                }
            });
        });
    }
}
