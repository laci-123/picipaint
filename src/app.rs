use eframe::egui::{self, Vec2};
use crate::paint_object::{freehand_curve::*, straight_line::*, *};
use crate::tool::Tool;


pub const WINDOW_INIT_SIZE: Vec2 = Vec2::new(800.0, 450.0);
pub const WINDOW_MIN_SIZE:  Vec2 = Vec2::new(300.0, 200.0);
pub const UI_SCALE: f32          = 1.5;
pub const NAME: &'static str     = "PiciPaint";


pub struct App {
    tools: Vec<Box<dyn Tool>>,
    active_tool_index: usize,
    objects: Vec<Box<dyn PaintObject>>,
}

impl App {
    pub fn new(_context: &eframe::CreationContext) -> Self {
        Self {
            tools: vec![
                Box::new(FreehandCurveTool::new(egui::Stroke::new(2.0, egui::Color32::BLUE))),
                Box::new(StraghtLineTool::new(egui::Stroke::new(3.0, egui::Color32::GREEN))),
            ],
            active_tool_index: 0,
            objects: vec![],
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_pixels_per_point(UI_SCALE);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                for (i, tool) in self.tools.iter().enumerate() {
                    let mut selected =  i == self.active_tool_index;
                    if ui.toggle_value(&mut selected, tool.display_name()).clicked() {
                        self.active_tool_index = i;
                    }
                }
            });

            egui::Frame::canvas(ui.style()).show(ui, |ui| {
                let size = ui.available_size();
                let (response, painter) = ui.allocate_painter(size, egui::Sense::click_and_drag());

                let active_tool = &mut self.tools[self.active_tool_index];
                active_tool.update(&response, &mut self.objects);
                active_tool.draw(&painter);

                for object in self.objects.iter() {
                    object.draw(&painter);
                }
            });
        });
    }
}
