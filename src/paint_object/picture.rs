use std::cell::OnceCell;
use image;
use eframe::egui;
use rfd::FileDialog;
use crate::egui_painter::EguiPainter;
use crate::engine::*;


pub struct Picture {
    bounding_rect: Rectangle,
    image: image::DynamicImage,
    image_name: String,
    texture: OnceCell<egui::TextureHandle>,
    mouse_pos: Vector2,
    selected: bool,
}

impl Picture {
    // OK(Some(picture)): the dropped file is a supported picture and we could read it sucessfully
    // OK(None):          the dropped file is not a picture in a supported format
    // Err(...):          the dropeed file is a supported picture but we could not read it because of some other reason
    pub fn from_dropped_file(dropped_file: &egui::DroppedFile) -> Result<Option<Self>, String> {
        let Some(file_path) = &dropped_file.path else {
            // This should never happen, `path` should only be `None` on the Wasm backend.
            return Err(format!("Error accessing dropped file. "));
        };

        let Some(file_extension) = file_path.extension() else {
            // We don't try to guess the format if the file doesn't have an extension,
            // just assume it isn't a supported image format.
            return Ok(None);
        };

        let mut format_is_supported = false;
        for format in image::ImageFormat::all() {
            if format.can_read() {
                for ext in format.extensions_str() {
                    if *ext == file_extension.to_string_lossy() {
                        format_is_supported = true;
                    }
                }
            }
        }
        if !format_is_supported {
            return Ok(None);
        }

        let image = image::ImageReader::open(&file_path)
                            .map_err(|err| err.to_string())?
                            .decode()
                            .map_err(|err| err.to_string())?;

        Ok(Some(Picture {
            bounding_rect: Rectangle::from_point_and_size(Vector2::zero(), image.width() as f32, image.height() as f32),
            image,
            image_name: file_path.to_string_lossy().into_owned(),
            texture: OnceCell::new(),
            mouse_pos: Vector2::zero(),
            selected: false,
        }))
    }
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
        painter.draw_image(self.bounding_rect, texture, camera);
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
        self.bounding_rect
    }

    fn shift_with(&mut self, p: Vector2) {
        self.bounding_rect.p1 += p;
        self.bounding_rect.p2 += p;
    }

    fn resize_to(&mut self, new_size: Rectangle) {
        self.bounding_rect = new_size;
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
                    bounding_rect: Rectangle::from_point_and_size(pos, image.width() as f32, image.height() as f32),
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
