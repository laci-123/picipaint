use eframe::egui;


#[derive(Default)]
pub struct ColorSelector {
    pub is_open: bool,
}

impl ColorSelector {
    pub fn update(&self, ctx: &egui::Context, color: &mut egui::Color32) {
        if self.is_open {
            egui::Window::new("Foreground color")
                .collapsible(false)
                .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
                .show(ctx, |ui| {
                    // If Color32's r, g, b and a fields were public then it would be much easier to do this.
                    let mut red = color.r();
                    let mut green = color.g();
                    let mut blue = color.b();
                    let alpha = color.a();
                    ui.add(egui::Slider::new(&mut red, 0..=255).text("red"));
                    ui.add(egui::Slider::new(&mut green, 0..=255).text("green"));
                    ui.add(egui::Slider::new(&mut blue, 0..=255).text("blue"));
                    *color = egui::Color32::from_rgba_premultiplied(red, green, blue, alpha);
            });
        }
    }
}
