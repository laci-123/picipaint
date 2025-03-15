use std::time::{Duration, Instant};
use eframe::egui;


pub struct FloatingWindow {
    pub is_open: bool,
    title: String,
    opening_time: Option<Instant>,
}

impl FloatingWindow {
    pub fn new(title: &str) -> Self {
        Self {
            is_open: false,
            opening_time: None,
            title: String::from(title),
        }
    }
    
    pub fn show(&mut self, ctx: &egui::Context, f: impl FnOnce(&mut egui::Ui)) {
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
            let mut window = egui::Window::new(&self.title).collapsible(false).resizable(false).title_bar(false);

            if Instant::now().duration_since(*opening_time) < Duration::from_millis(100) {
                window = window.anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO);
            }

            let response = window.show(ctx, f);

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
        }
    }
}

