use std::cell::OnceCell;
use image;
use eframe::egui;
use crate::egui_painter::EguiPainter;
use crate::engine::*;


pub struct Picture {
    top_left: Vector2,
    image: image::DynamicImage,
    texture: OnceCell<egui::TextureHandle>,
    mouse_pos: Vector2,
    selected: bool,
}

impl PaintObject<EguiPainter> for Picture {
    fn update(&mut self, input: &UserInput, camera: &Camera) {
        match input {
            UserInput::MouseMove { position, .. } => {
                self.mouse_pos = camera.convert_to_world_coordinates(*position);
            },
            UserInput::MouseClick { position, .. } => {
                self.mouse_pos = camera.convert_to_world_coordinates(*position);
            },
            _ => {
                // do nothing
            },
        }
    }
    
    fn draw<'a>(&self, painter: &mut WorldPainter<'a, EguiPainter>, camera: &Camera) {
        let texture = self.texture.get_or_init(|| {
            painter.load_image("cica", &self.image)
        });
        painter.draw_image(self.get_bounding_rect(), texture, camera);
    }
    
    fn is_selected(&self) -> bool {
        self.selected
    }
    
    fn set_selected(&mut self, value: bool) {
        self.selected = value;
    }
    
    fn is_under_mouse(&self) -> bool {
        self.get_bounding_rect().contains_point(self.mouse_pos)
    }
    
    fn get_bounding_rect(&self) -> Rectangle {
        Rectangle::from_point_and_size(self.top_left, self.image.width() as f32, self.image.height() as f32)
    }
}


#[derive(Default)]
pub struct PictureTool {
    // nothing
}

impl Tool<EguiPainter> for PictureTool {
    fn update(&mut self, input: &UserInput, objects: &mut Vec<Box<dyn PaintObject<EguiPainter>>>, _stroke: Stroke, camera: &Camera) {
        if let UserInput::MouseClick { position, .. } = input {
            let image = image::ImageReader::open(&std::path::Path::new("../cat.png")).unwrap().decode().unwrap();
            let pos   = camera.convert_to_world_coordinates(*position);
            objects.push(Box::new(Picture {
                top_left: pos,
                image,
                texture: OnceCell::new(),
                mouse_pos: pos,
                selected: false,
            }));
        }
    }
    
    fn draw<'a>(&self, _painter: &mut WorldPainter<'a, EguiPainter>, _camera: &Camera) {
        // nothing
    }
    
    fn display_name(&self) -> &str {
        "insert picture"
    }
}
