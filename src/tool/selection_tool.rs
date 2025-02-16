use super::*;


pub struct SelectionTool {}

impl SelectionTool {
    pub fn new() -> Self {
        Self{}
    }
}

impl Tool for SelectionTool {
    fn update(&mut self, response: &egui::Response, objects: &mut Vec<Box<dyn PaintObject>>) {
        if let Some(mouse_pos) = response.interact_pointer_pos() {
            for object in objects.iter_mut() {
                object.set_selected(object.is_under_mouse(mouse_pos.to_vec2()));
            }
        }
    }

    fn draw(&self, _painter: &egui::Painter) {}
    
    fn display_name(&self) -> &str {
        "selection"
    }
}
