use eframe::egui;


pub trait PaintObject {
    fn draw(&self, painter: &egui::Painter);
    fn is_selected(&self) -> bool;
    fn set_selected(&mut self, value: bool);
    fn is_under_mouse(&self, mouse_pos: egui::Vec2) -> bool;
    fn get_bounding_rect(&self) -> egui::Rect;

    fn draw_selection(&self, painter: &egui::Painter) {
        if self.is_selected() {
            painter.rect_stroke(self.get_bounding_rect(), egui::Rounding::ZERO, egui::Stroke::new(1.0, egui::Color32::WHITE));
        }
    }
}


pub mod straight_line;
pub mod freehand_curve;
