use eframe::egui;
use crate::paint_object::PaintObject;
use crate::view_transform::ViewTransform;


pub trait Tool {
    fn update(&mut self, response: &egui::Response, transform: &ViewTransform, objects: &mut Vec<Box<dyn PaintObject>>, stroke: egui::Stroke);
    fn draw(&self, transform: &ViewTransform, painter: &egui::Painter);
    fn before_deactivate(&mut self, objects: &mut Vec<Box<dyn PaintObject>>);
    fn display_name(&self) -> &str;
}


pub mod selection_tool;
