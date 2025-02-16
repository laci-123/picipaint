use eframe::egui;
use crate::tool::Tool;
use super::*;


pub struct StraightLine {
    start: egui::Pos2,
    end: egui::Pos2,
    stroke: egui::Stroke,
    selected: bool,
}

impl PaintObject for StraightLine {
    fn draw(&self, painter: &egui::Painter) {
        painter.line_segment([self.start, self.end], self.stroke);
    }

    fn is_selected(&self) -> bool {
        self.selected
    }

    fn set_selected(&mut self, value: bool) {
        self.selected = value;
    }

    fn is_under_mouse(&self, mouse_pos: egui::Vec2) -> bool {
        let length           = (self.end - self.start).length();
        let mouse_from_start = (mouse_pos - self.start.to_vec2()).length();
        let mouse_from_end   = (mouse_pos - self.end.to_vec2()).length();

        mouse_from_start + mouse_from_end < length + 0.5
    }

    fn get_bounding_rect(&self) -> egui::Rect {
        egui::Rect::from_two_pos(self.start, self.end)
    }
}


pub struct StraghtLineTool {
    start: Option<egui::Pos2>,
    end: Option<egui::Pos2>,
    stroke: egui::Stroke,
}

impl StraghtLineTool {
    pub fn new(stroke: egui::Stroke) -> Self{
        Self {
            start: None,
            end: None,
            stroke,
        }
    }
}

impl Tool for StraghtLineTool {
    fn update(&mut self, response: &egui::Response, objects: &mut Vec<Box<dyn PaintObject>>) {
        match self.start {
            None => {
                if response.clicked_by(egui::PointerButton::Primary) {
                    self.start = response.interact_pointer_pos();
                }
            },
            Some(start) => {
                if let Some(end) = response.hover_pos() {
                    self.end = Some(end);
                }

                if let Some(end) = self.end {
                    if response.clicked_by(egui::PointerButton::Primary) {
                        self.start = None;
                        self.end = None;
                        objects.push(Box::new(StraightLine {
                            start,
                            end,
                            stroke: self.stroke,
                            selected: false,
                        }));
                    }
                }
            },
        }
    }

    fn draw(&self, painter: &egui::Painter) {
        if let (Some(start), Some(end)) = (self.start, self.end) {
            painter.line_segment([start, end], self.stroke);
        }
    }

    fn before_deactivate(&mut self, _objects: &mut Vec<Box<dyn PaintObject>>) {}

    fn display_name(&self) -> &str {
        "straight line"
    }
}

