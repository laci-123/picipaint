use eframe::egui;
use crate::engine;


impl engine::ScreenPainter for egui::Painter {
    fn draw_line(&mut self, start: engine::Vector2, end: engine::Vector2, stroke: engine::Stroke) {
        self.line_segment([egui::Pos2::from(start), egui::Pos2::from(end)], egui::Stroke::from(stroke));
    }
    
    fn draw_circle(&mut self, center: engine::Vector2, radius: f32, stroke: engine::Stroke) {
        self.circle_filled(egui::Pos2::from(center), radius, stroke.color);
    }
    
    fn draw_rectangle(&mut self, rectangle: engine::Rectangle, stroke: engine::Stroke) {
        let rect = egui::Rect::from_min_max(egui::Pos2::from(rectangle.p1), egui::Pos2::from(rectangle.p2));
        let corner_rounding = 0.0;
        self.rect_stroke(rect, corner_rounding, stroke);
    }
    
    fn draw_rectangle_filled(&mut self, rectangle: engine::Rectangle, color: engine::Color, stroke: Option<engine::Stroke>) {
        let rect = egui::Rect::from_min_max(egui::Pos2::from(rectangle.p1), egui::Pos2::from(rectangle.p2));
        let corner_rounding = 0.0;
        if let Some(s) = stroke {
            self.rect(rect, corner_rounding, color, s);
        }
        else {
            self.rect_filled(rect, corner_rounding, color);
        }
    }
}


impl From<engine::Vector2> for egui::Pos2 {
    fn from(other: engine::Vector2) -> egui::Pos2 {
        egui::Pos2 {
            x: other.x,
            y: other.y,
        }
    }
}


impl From<engine::Color> for egui::Color32 {
    fn from(other: engine::Color) -> egui::Color32 {
        egui::Color32::from_rgba_premultiplied(other.red, other.green, other.blue, other.alpha)
    }
}


impl From<engine::Stroke> for egui::Stroke {
    fn from(other: engine::Stroke) -> egui::Stroke {
        egui::Stroke::new(other.thickness, other.color)
    }
}
