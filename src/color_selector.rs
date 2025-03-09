use std::time::{Duration, Instant};
use crate::engine::*;
use eframe::egui;


pub struct ColorSelector {
    pub is_open: bool,
    opening_time: Option<Instant>,
    caption: String,
    old_color: Option<Color>,
}

impl ColorSelector {
    pub fn new(caption: &str) -> Self {
        Self {
            is_open: false,
            opening_time: None,
            caption: String::from(caption),
            old_color: None,
        }
    }
    
    pub fn update(&mut self, ctx: &egui::Context, color: &mut Color) {
        ctx.input(|input| {
            if input.key_down(egui::Key::Escape) {
                self.is_open = false;
            }
        });

        if self.is_open {
            // Goal: open the window centered but allow moving it. But...
            // 
            // window.default_pos(...) doesn't work for some reason,
            // and window.anchor(...) disables moving the window,
            // so we have to do it like this.
            // It also doesn't work to just do window.anchor(...) on only the
            // first frame after the window's been opened.
            // We have to actually wait for a little while.

            let opening_time = self.opening_time.get_or_insert(Instant::now());
            let mut window = egui::Window::new(&self.caption).collapsible(false).resizable(false);

            if Instant::now().duration_since(*opening_time) < Duration::from_millis(100) {
                window = window.anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO);
            }

            let response = window.show(ctx, |ui| {
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

            if let Some(r) = response {
                // Have to wait a little bit to not register the click that opened this window as clicking elsewhere.
                let been_open_for_a_while = Instant::now().duration_since(*opening_time) > Duration::from_millis(100);
                if r.response.clicked_elsewhere() && been_open_for_a_while {
                    self.is_open = false;
                }
            }
        }
        else {
            self.opening_time = None;
            self.old_color = None;
        }
    }
}
