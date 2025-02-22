use eframe::egui::{self, Vec2};
use crate::color_selector::ColorSelector;
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
    stroke: egui::Stroke,
    bg_color: egui::Color32,
    fg_color_selector: ColorSelector,
    bg_color_selector: ColorSelector,
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
            stroke: egui::Stroke::new(2.0, egui::Color32::BLUE),
            bg_color: egui::Color32::BLACK,
            fg_color_selector: ColorSelector::new("Foreground color"),
            bg_color_selector: ColorSelector::new("Background color"),
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

                ui.toggle_value(&mut self.fg_color_selector.is_open, "fg color");
                ui.toggle_value(&mut self.bg_color_selector.is_open, "bg color");
                ui.add(egui::Slider::new(&mut self.stroke.width, 0.5..=10.0)).on_hover_ui_at_pointer(|ui| {
                    ui.label("line thickness");
                });
            });

            ui.separator();

            egui::Frame::canvas(ui.style()).show(ui, |ui| {
                let size = ui.available_size();
                let (response, painter) = ui.allocate_painter(size, egui::Sense::click_and_drag());

                painter.rect_filled(response.rect, 0.0, self.bg_color);
                let active_tool = &mut self.tools[self.active_tool_index];
                active_tool.update(&response, &mut self.objects, self.stroke);
                active_tool.draw(&painter);

                for object in self.objects.iter() {
                    object.draw(&painter);
                    object.draw_selection(&painter);
                }
            });

            self.fg_color_selector.update(ctx, &mut self.stroke.color);
            self.bg_color_selector.update(ctx, &mut self.bg_color);
        });
    }
}
