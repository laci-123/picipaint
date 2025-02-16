use eframe::egui;
use crate::tool::Tool;
use super::*;


pub struct FreehandCurve {
    points: Vec<egui::Pos2>,
    stroke: egui::Stroke,
    min_x: f32,
    max_x: f32,
    min_y: f32,
    max_y: f32,
    selected: bool,
}

impl PaintObject for FreehandCurve {
    fn draw(&self, painter: &egui::Painter) {
        for p1p2 in self.points.windows(2) {
            let [p1, p2] = p1p2 else {unreachable!()};
            painter.line_segment([*p1, *p2], self.stroke);
        }
    }

    fn is_selected(&self) -> bool {
        self.selected
    }
    
    fn set_selected(&mut self, value: bool) {
        self.selected = value;
    }
    
    fn is_under_mouse(&self, mouse_pos: egui::Vec2) -> bool {
        let in_bounding_rect = self.min_x < mouse_pos.x && mouse_pos.x < self.max_x &&
                               self.min_y < mouse_pos.y && mouse_pos.y < self.max_y;
        if in_bounding_rect {
            for point in self.points.iter() {
                if (*point - mouse_pos).to_vec2().length_sq() < 10.0 {
                    return true;
                }
            }
        }
        return false;
    }
}


pub struct FreehandCurveTool {
    curve: FreehandCurve,
}

impl FreehandCurveTool {
    pub fn new(stroke: egui::Stroke) -> Self {
        Self {
            curve: Self::new_curve(stroke),
        }
    }

    fn new_curve(stroke: egui::Stroke) -> FreehandCurve {
        FreehandCurve {
            stroke,
            points: Vec::new(),
            min_x: f32::INFINITY,
            min_y: f32::INFINITY,
            max_x: f32::NEG_INFINITY,
            max_y: f32::NEG_INFINITY,
            selected: false,
        }
    }
}

impl Tool for FreehandCurveTool {
    fn update(&mut self, response: &egui::Response, objects: &mut Vec<Box<dyn PaintObject>>) {
        if response.dragged_by(egui::PointerButton::Primary) {
            if let Some(point) = response.interact_pointer_pos() {
                self.curve.min_x = self.curve.min_x.min(point.x);
                self.curve.min_y = self.curve.min_y.min(point.y);
                self.curve.max_x = self.curve.max_x.max(point.x);
                self.curve.max_y = self.curve.max_y.max(point.y);
                self.curve.points.push(point);
            }
        }
        else if self.curve.points.len() > 0 {
            let new_curve = Self::new_curve(self.curve.stroke);
            objects.push(Box::new(std::mem::replace(&mut self.curve, new_curve)));
        }
    }

    fn draw(&self, painter: &egui::Painter) {
        self.curve.draw(painter);
    }

    fn display_name(&self) -> &str {
        "curve"
    }
}
