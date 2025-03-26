use std::cell::OnceCell;
use image;
use eframe::egui;
use rfd::FileDialog;
use crate::egui_painter::EguiPainter;
use crate::engine::*;


pub struct Picture {
    top_left: Vector2,
    image: image::DynamicImage,
    image_name: String,
    texture: OnceCell<egui::TextureHandle>,
    mouse_pos: Vector2,
    selected: bool,
}

impl PaintObject<EguiPainter> for Picture {
    fn update(&mut self, input: &UserInput, camera: &Camera) {
        if let Some(position) = input.mouse_position() {
            self.mouse_pos = camera.convert_to_world_coordinates(position);
        }
    }
    
    fn draw<'a>(&self, painter: &mut WorldPainter<'a, EguiPainter>, camera: &Camera) {
        let texture = self.texture.get_or_init(|| {
            painter.load_image(&self.image_name, &self.image)
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


pub struct PictureTool {
    icon: egui::ImageSource<'static>,
}

impl Default for PictureTool {
    fn default() -> Self {
        Self {
            icon: egui::include_image!("../../img/picture_tool.png"),
        }
    }
}

impl Tool<EguiPainter, egui::ImageSource<'static>> for PictureTool {
    fn update(&mut self, input: &UserInput, _stroke: Stroke, camera: &Camera) -> Result<Option<Box<dyn PaintObject<EguiPainter>>>, String> {
        if let UserInput::MouseClick { position, .. } = input {
            if let Some(file_path) = FileDialog::new().add_filter("png images", &["png"]).pick_file() {
                let image = image::ImageReader::open(&file_path)
                                 .map_err(|err| err.to_string())?
                                 .decode()
                                 .map_err(|err| err.to_string())?;
                let pos   = camera.convert_to_world_coordinates(*position);
                return Ok(Some(Box::new(Picture {
                    top_left: pos,
                    image,
                    image_name: file_path.to_string_lossy().into_owned(),
                    texture: OnceCell::new(),
                    mouse_pos: pos,
                    selected: false,
                })));
            }
        }

        return Ok(None);
    }
    
    fn draw<'a>(&self, _painter: &mut WorldPainter<'a, EguiPainter>, _camera: &Camera) {
        // nothing
    }
    
    fn display_name(&self) -> &str {
        "insert picture"
    }

    fn icon(&self) -> egui::ImageSource<'static> {
        self.icon.clone()
    }
}
