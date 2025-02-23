use eframe::egui;


pub struct ViewTransform {
    pub scale: f32,
    pub shift: egui::Vec2,
}

impl Default for ViewTransform {
    fn default() -> Self {
        Self {
            scale: 1.0,
            shift: egui::Vec2::new(0.0, 0.0),
        }
    }
}

impl ViewTransform {
    pub fn screen_to_world(&self, point: egui::Pos2) -> egui::Pos2 {
        (point - self.shift) * self.scale
    }

    pub fn world_to_screen(&self, point: egui::Pos2) -> egui::Pos2 {
        point * (1.0 / self.scale) + self.shift
    }

    pub fn update(&mut self, response: &egui::Response) {
        if response.contains_pointer() {
            response.ctx.input(|input| {
                if input.smooth_scroll_delta.y > 0.0 {
                    self.scale *= 0.99;
                }
                else if input.smooth_scroll_delta.y < 0.0 {
                    self.scale *= 1.01;
                }
            })
        }
        if response.dragged_by(egui::PointerButton::Middle) {
            self.shift += response.drag_delta();
        }
    }
}
