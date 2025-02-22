use eframe::egui;
use std::time::{Duration, Instant};


#[derive(Default)]
pub struct ColorSelector {
    pub is_open: bool,
    opening_time: Option<Instant>,
}

impl ColorSelector {
    pub fn update(&mut self, ctx: &egui::Context, color: &mut egui::Color32) {
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
            
            let mut window = egui::Window::new("Foreground color").collapsible(false).resizable(false);

            if Instant::now().duration_since(*opening_time) < Duration::from_millis(100) {
                window = window.anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO);
            }

            let response = window.show(ctx, |ui| {
                egui::Frame::canvas(ui.style()).show(ui, |ui| {
                    let size = egui::Vec2::new(100.0, 100.0);
                    let (response, painter) = ui.allocate_painter(size, egui::Sense::hover());
                    painter.rect_filled(response.rect, 0.0, *color);
                });

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

            if let Some(r) = response {
                if r.response.clicked_elsewhere() && Instant::now().duration_since(*opening_time) > Duration::from_millis(100) {
                    self.is_open = false;
                }
            }
        }
        else {
            self.opening_time = None;
        }
    }
}
