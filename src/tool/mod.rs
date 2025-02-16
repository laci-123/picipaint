use eframe::egui;
use crate::paint_object::PaintObject;


pub trait Tool {
    fn update(&mut self, response: &egui::Response, objects: &mut Vec<Box<dyn PaintObject>>);
    fn draw(&self, painter: &egui::Painter);
    fn display_name(&self) -> &str;
}


pub mod selection_tool;
