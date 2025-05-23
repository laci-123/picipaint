use crate::egui_painter::EguiPainter;
use crate::primitives::*;
use crate::engine::*;
use eframe::egui;


pub struct StraightLine {
    base: PaintObjectCommon,
    start: Vector2<WorldSpace>,
    end: Vector2<WorldSpace>,
    stroke: Stroke<WorldSpace>,
    mouse_pos: Vector2<WorldSpace>,
}

impl PaintObject<EguiPainter> for StraightLine {
    fn base(&self) -> &PaintObjectCommon {
        &self.base
    }

    fn base_mut(&mut self) -> &mut PaintObjectCommon {
        &mut self.base
    }
    
    fn update(&mut self, input: &UserInput, camera: &Camera) {
        if let Some(position) = input.mouse_position() {
            self.mouse_pos = camera.point_to_world_coordinates(position);
        }
    }
    
    fn draw<'a>(&self, painter: &mut WorldPainter<'a, EguiPainter>, camera: &Camera) {
        painter.draw_line(self.start, self.end, self.stroke, camera);
    }
    
    fn is_under_mouse(&self) -> bool {
        let epsilon = 10.0;
        (self.start - self.mouse_pos).length() + (self.end - self.mouse_pos).length() < (self.end - self.start).length() + epsilon
    }
    
    fn get_bounding_rect(&self) -> Rectangle<WorldSpace> {
        Rectangle::from_points_well_ordered(self.start, self.end)
    }

    fn shift_with(&mut self, p: Vector2<WorldSpace>) {
        self.start = self.start + p;
        self.end   = self.end + p;
    }

    fn resize_to(&mut self, new_size: Rectangle<WorldSpace>) {
        self.start = new_size.p1;
        self.end = new_size.p2;
    }

    fn clip_to(&mut self, new_size: Rectangle<WorldSpace>) {
        let mut new_start = None;
        let mut new_end = None;
        for int_point in new_size.intersection_with_line(self.start, self.end) {
            if int_point.is_some() {
                if new_start.is_none() {
                    new_start = int_point;
                }
                else {
                    new_end = int_point;
                }
            }
        }
        if let Some(p) = new_start {
            self.start = p;
        }
        if let Some(p) = new_end {
            self.end = p;
        }
    }
}


pub struct StraghtLineTool {
    start: Option<Vector2<WorldSpace>>,
    stroke: Option<Stroke<WorldSpace>>, // Only optional because Stroke doesn't have a default value, so we have to wait until the first call to `update` to set it.
    mouse_pos: Vector2<WorldSpace>,
    icon: egui::ImageSource<'static>,
}

impl Default for StraghtLineTool {
    fn default() -> Self{
        Self {
            start: None,
            stroke: None,
            mouse_pos: Vector2::zero(),
            icon: egui::include_image!("../../img/straightline_tool.png"),
        }
    }
}

impl Tool<EguiPainter, egui::ImageSource<'static>> for StraghtLineTool {
    fn update(&mut self, input: &UserInput, stroke: Stroke<WorldSpace>, camera: &Camera) -> Result<Option<Box<dyn PaintObject<EguiPainter>>>, String> {
        self.stroke = Some(stroke);
        
        match input {
            UserInput::MouseMove { position, .. } => {
                self.mouse_pos = camera.point_to_world_coordinates(*position);
            },
            UserInput::MouseClick { position, button: MouseButton::Left, is_shift_down: false } => {
                let p = camera.point_to_world_coordinates(*position);
                if let Some(start) = self.start {
                    let line = StraightLine {
                        base: PaintObjectCommon::default(),
                        start,
                        end: p,
                        stroke,
                        mouse_pos: p,
                    };
                    self.start = None;
                    return Ok(Some(Box::new(line)));
                }
                else {
                    self.start = Some(p);
                }
                self.mouse_pos = p;
            },
            _ => {
                // do nothing
            },
        }

        return Ok(None);
    }
    
    fn draw<'a>(&self, painter: &mut WorldPainter<'a, EguiPainter>, _bg_color: Color, camera: &Camera) {
        if let Some(stroke) = self.stroke {
            if let Some(start) = self.start {
                painter.draw_line(start, self.mouse_pos, stroke, camera);
            }
        }
    }
    
    fn display_name(&self) -> &str {
        "straight line"
    }

    fn icon(&self) -> egui::ImageSource<'static> {
        self.icon.clone()
    }
}

