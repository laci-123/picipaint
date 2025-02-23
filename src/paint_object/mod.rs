use eframe::egui;
use crate::view_transform::ViewTransform;


pub trait PaintObject {
    fn draw(&self, transform: &ViewTransform, painter: &egui::Painter);
    fn is_selected(&self) -> bool;
    fn set_selected(&mut self, value: bool);
    fn is_under_mouse(&self, mouse_pos: egui::Vec2) -> bool;
    fn get_bounding_rect(&self) -> egui::Rect;

    fn draw_selection(&self, tr: &ViewTransform, painter: &egui::Painter) {
        if self.is_selected() {
            let rect = self.get_bounding_rect();
            let tr_rect = egui::Rect::from_min_max(tr.world_to_screen(rect.min), tr.world_to_screen(rect.max));
            painter.rect_stroke(tr_rect, egui::Rounding::ZERO, egui::Stroke::new(1.0, egui::Color32::WHITE));
        }
    }
}


pub mod straight_line;
pub mod freehand_curve;
