use eframe::egui::{self, Vec2};
use crate::paint_object::{freehand_curve::*, straight_line::*, *};
use crate::tool::{Tool, selection_tool::*};


pub const WINDOW_INIT_SIZE: Vec2 = Vec2::new(1000.0, 600.0);
pub const WINDOW_MIN_SIZE:  Vec2 = Vec2::new(300.0, 200.0);
pub const UI_SCALE: f32          = 1.5;
pub const NAME: &'static str     = "PiciPaint";


pub struct App {
    tools: Vec<Box<dyn Tool>>,
    active_tool_index: usize,
    objects: Vec<Box<dyn PaintObject>>,
    fg_color_red: u8,
    fg_color_green: u8,
    fg_color_blue: u8,
    line_thickness: f32,
    fg_color_window_is_open: bool,
}

impl App {
    pub fn new(_context: &eframe::CreationContext) -> Self {
        Self {
            tools: vec![
                Box::new(SelectionTool::new()),
                Box::new(FreehandCurveTool::new()),
                Box::new(StraghtLineTool::new()),
            ],
            active_tool_index: 0,
            objects: vec![],
            fg_color_red: 0,
            fg_color_green: 0,
            fg_color_blue: 255,
            line_thickness: 1.0,
            fg_color_window_is_open: false,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_pixels_per_point(UI_SCALE);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                let mut old_active_tool_index = None;
                
                for (i, tool) in self.tools.iter().enumerate() {
                    let mut selected =  i == self.active_tool_index;
                    if ui.toggle_value(&mut selected, tool.display_name()).clicked() {
                        old_active_tool_index = Some(self.active_tool_index);
                        self.active_tool_index = i;
                    }
                }

                if let Some(old_index) = old_active_tool_index {
                    self.tools[old_index].before_deactivate(&mut self.objects);
                }

                ui.separator();

                ui.toggle_value(&mut self.fg_color_window_is_open, "color");
                ui.add(egui::Slider::new(&mut self.line_thickness, 0.5..=10.0)).on_hover_ui_at_pointer(|ui| {
                    ui.label("line thickness");
                });
            });

            ui.separator();

            egui::Frame::canvas(ui.style()).show(ui, |ui| {
                let size = ui.available_size();
                let (response, painter) = ui.allocate_painter(size, egui::Sense::click_and_drag());

                let active_tool = &mut self.tools[self.active_tool_index];
                let stroke = egui::Stroke::new(self.line_thickness, egui::Color32::from_rgb(self.fg_color_red, self.fg_color_green, self.fg_color_blue));
                active_tool.update(&response, &mut self.objects, stroke);
                active_tool.draw(&painter);

                for object in self.objects.iter() {
                    object.draw(&painter);
                    object.draw_selection(&painter);
                }
            });

            if self.fg_color_window_is_open {
                egui::Window::new("Foreground color")
                    .collapsible(false)
                    .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
                    .show(ctx, |ui| {
                        ui.add(egui::Slider::new(&mut self.fg_color_red, 0..=255).text("red"));
                        ui.add(egui::Slider::new(&mut self.fg_color_green, 0..=255).text("green"));
                        ui.add(egui::Slider::new(&mut self.fg_color_blue, 0..=255).text("blue"));
                });
            }
        });
    }
}
