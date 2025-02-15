use eframe::egui;
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

        mouse_from_start + mouse_from_end < length + 10.0
    }
}


pub struct StraightLineMaker {
    start: Option<egui::Pos2>,
    end: Option<egui::Pos2>,
    stroke: egui::Stroke,
}

impl StraightLineMaker {
    pub fn new(stroke: egui::Stroke) -> Self{
        Self {
            start: None,
            end: None,
            stroke,
        }
    }
}

impl PaintObjectMaker<StraightLine> for StraightLineMaker {
    fn update(&mut self, response: &egui::Response) -> Option<StraightLine> {
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
                        return Some(StraightLine {
                            start,
                            end,
                            stroke: self.stroke,
                            selected: false,
                        });
                    }
                }
            },
        }

        return None;
    }

    fn draw(&self, painter: &egui::Painter) {
        if let (Some(start), Some(end)) = (self.start, self.end) {
            painter.line_segment([start, end], self.stroke);
        }
    }
}

