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
    fn draw(&self, tr: &ViewTransform, painter: &egui::Painter) {
        for p1p2 in self.points.windows(2) {
            let p1 = tr.world_to_screen(p1p2[0]);
            let p2 = tr.world_to_screen(p1p2[1]);
            painter.line_segment([p1, p2], self.stroke);
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

    fn get_bounding_rect(&self) -> egui::Rect {
        egui::Rect::from_min_max(egui::Pos2::new(self.min_x, self.min_y), egui::Pos2::new(self.max_x, self.max_y))
    }
}


pub struct FreehandCurveTool {
    curve: FreehandCurve,
}

impl FreehandCurveTool {
    pub fn new() -> Self {
        Self {
            curve: Self::new_curve(),
        }
    }

    fn new_curve() -> FreehandCurve {
        FreehandCurve {
            stroke: egui::Stroke::default(), // this will always be overwritten in update
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
    fn update(&mut self, response: &egui::Response, tr: &ViewTransform, objects: &mut Vec<Box<dyn PaintObject>>, stroke: egui::Stroke) {
        if response.contains_pointer() {
            response.ctx.output_mut(|output| {
                output.cursor_icon = egui::CursorIcon::Crosshair;
            });
        }

        self.curve.stroke = stroke;

        if response.dragged_by(egui::PointerButton::Primary) {
            if let Some(point) = response.interact_pointer_pos() {
                let p = tr.screen_to_world(point);
                self.curve.min_x = self.curve.min_x.min(p.x);
                self.curve.min_y = self.curve.min_y.min(p.y);
                self.curve.max_x = self.curve.max_x.max(p.x);
                self.curve.max_y = self.curve.max_y.max(p.y);
                self.curve.points.push(p);
            }
        }
        else if self.curve.points.len() > 0 {
            objects.push(Box::new(std::mem::replace(&mut self.curve, Self::new_curve())));
        }
    }

    fn draw(&self, tr: &ViewTransform, painter: &egui::Painter) {
        self.curve.draw(tr, painter);
    }

    fn before_deactivate(&mut self, _objects: &mut Vec<Box<dyn PaintObject>>) {}

    fn display_name(&self) -> &str {
        "curve"
    }
}
