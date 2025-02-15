use eframe::egui;


pub trait PaintObject {
    fn draw(&self, painter: &egui::Painter);
    fn is_selected(&self) -> bool;
    fn set_selected(&mut self, value: bool);
    fn is_under_mouse(&self, mouse_pos: egui::Vec2) -> bool;
}


pub trait PaintObjectMaker<PO> {
    fn update(&mut self, response: &egui::Response) -> Option<PO>;
    fn draw(&self, painter: &egui::Painter);
}


pub mod straight_line;
//pub mod freehand_curve;
