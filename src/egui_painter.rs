use eframe::egui;
use crate::engine;


pub struct EguiPainter {
    painter: egui::Painter,
    context: egui::Context,
}


impl engine::ScreenPainter for EguiPainter {
    type Texture = egui::TextureHandle;
    
    fn draw_line(&mut self, start: engine::Vector2, end: engine::Vector2, stroke: engine::Stroke) {
        self.painter.line_segment([egui::Pos2::from(start), egui::Pos2::from(end)], egui::Stroke::from(stroke));
    }
    
    fn draw_circle(&mut self, center: engine::Vector2, radius: f32, stroke: engine::Stroke) {
        self.painter.circle_filled(egui::Pos2::from(center), radius, stroke.color);
    }
    
    fn draw_rectangle(&mut self, rectangle: engine::Rectangle, stroke: engine::Stroke) {
        let corner_rounding = 0.0;
        self.painter.rect_stroke(egui::Rect::from(rectangle), corner_rounding, stroke);
    }
    
    fn draw_rectangle_filled(&mut self, rectangle: engine::Rectangle, color: engine::Color, stroke: Option<engine::Stroke>) {
        let rect = egui::Rect::from(rectangle);
        let corner_rounding = 0.0;
        if let Some(s) = stroke {
            self.painter.rect(rect, corner_rounding, color, s);
        }
        else {
            self.painter.rect_filled(rect, corner_rounding, color);
        }
    }

    fn load_image(&mut self, name: &str, image: &image::DynamicImage) -> Self::Texture {
        let size = [image.width() as usize, image.height() as usize];
        let image_buffer = image.to_rgba8();
        let pixels = image_buffer.as_flat_samples();
        let color_image = egui::ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());
        self.context.load_texture(name, color_image, egui::TextureOptions::default())
    }
    
    fn draw_image(&mut self, frame: engine::Rectangle, texture: &Self::Texture) {
            self.painter.image(texture.id(),
                               egui::Rect::from(frame),
                               egui::Rect::from_min_max(egui::Pos2 { x: 0.0, y: 0.0 }, egui::Pos2 { x: 1.0, y: 1.0 }),
                               egui::Color32::WHITE);
    }
}

impl EguiPainter {
    pub fn new(painter: egui::Painter, context: egui::Context) -> Self {
        Self {
            painter,
            context,
        }
    }
}


impl From<engine::Vector2> for egui::Pos2 {
    fn from(other: engine::Vector2) -> egui::Pos2 {
        egui::Pos2 {
            x: other.x,
            y: other.y,
        }
    }
}


impl From<egui::Pos2> for engine::Vector2 {
    fn from(other: egui::Pos2) -> engine::Vector2 {
        engine::Vector2 {
            x: other.x,
            y: other.y,
        }
    }
}


impl From<egui::Vec2> for engine::Vector2 {
    fn from(other: egui::Vec2) -> engine::Vector2 {
        engine::Vector2 {
            x: other.x,
            y: other.y,
        }
    }
}


impl From<engine::Color> for egui::Color32 {
    fn from(other: engine::Color) -> egui::Color32 {
        egui::Color32::from_rgba_premultiplied(other.red, other.green, other.blue, other.alpha)
    }
}


impl From<engine::Stroke> for egui::Stroke {
    fn from(other: engine::Stroke) -> egui::Stroke {
        egui::Stroke::new(other.thickness, other.color)
    }
}


impl From<engine::Rectangle> for egui::Rect {
    fn from(other: engine::Rectangle) -> egui::Rect {
        egui::Rect::from_min_max(egui::Pos2::from(other.p1), egui::Pos2::from(other.p2))
    }
}
