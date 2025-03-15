use crate::{engine::*, floating_window::FloatingWindow};
use eframe::egui;


pub struct ColorSelector {
    old_color: Option<Color>,
    title: String,
    pub window: FloatingWindow,
}

impl ColorSelector {
    pub fn new(title: &str) -> Self {
        Self {
            title: String::from(title),
            window: FloatingWindow::new(title),
            old_color: None,
        }
    }
    
    pub fn update(&mut self, ctx: &egui::Context, color: &mut Color) {
        self.window.show(ctx, |ui| {
            ui.heading(&self.title);
            let old_color = self.old_color.get_or_insert(*color);
            egui::Frame::canvas(ui.style()).show(ui, |ui| {
                let w = 100.0;
                let h = 100.0;
                let (response, painter) = ui.allocate_painter(egui::Vec2::new(w, h), egui::Sense::hover());
                let origin = response.rect.min;
                painter.rect_filled(egui::Rect::from_min_size(origin,                                 egui::Vec2::new(w / 2.0, h)), 0.0, *old_color);
                painter.rect_filled(egui::Rect::from_min_size(origin + egui::Vec2::new(w / 2.0, 0.0), egui::Vec2::new(w / 2.0, h)), 0.0, *color);
            });

            ui.add(egui::Slider::new(&mut color.red, 0..=255).text("red"));
            ui.add(egui::Slider::new(&mut color.green, 0..=255).text("green"));
            ui.add(egui::Slider::new(&mut color.blue, 0..=255).text("blue"));
        });

        if !self.window.is_open {
            self.old_color = None;
        }
    }
}
