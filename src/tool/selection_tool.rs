use super::*;


pub struct SelectionTool {}

impl SelectionTool {
    pub fn new() -> Self {
        Self{}
    }
}

impl Tool for SelectionTool {
    fn update(&mut self, response: &egui::Response, objects: &mut Vec<Box<dyn PaintObject>>, _stroke: egui::Stroke) {
        let is_shift_down =
        response.ctx.input(|input| {
            input.modifiers.matches_exact(egui::Modifiers::SHIFT)
        });

        if response.clicked_by(egui::PointerButton::Primary) {
            if let Some(mouse_pos) = response.interact_pointer_pos() {
                for object in objects.iter_mut() {
                    let is_under_mouse = object.is_under_mouse(mouse_pos.to_vec2());

                    if is_shift_down {
                        if is_under_mouse {
                            object.set_selected(!object.is_selected());
                        }
                    }
                    else {
                        object.set_selected(is_under_mouse);
                    }
                }
            }
        }
    }

    fn draw(&self, _painter: &egui::Painter) {}

    fn before_deactivate(&mut self, objects: &mut Vec<Box<dyn PaintObject>>) {
        for object in objects.iter_mut() {
            object.set_selected(false);
        }
    }
    
    fn display_name(&self) -> &str {
        "selection"
    }
}
