use crate::primitives::*;
use crate::engine::*;
use crate::egui_painter::EguiPainter;
use eframe::egui;


pub struct FreehandCurve {
    points: Vec<Vector2<WorldSpace>>,
    stroke: Option<Stroke>, // Only optional because Stroke doesn't have a default value, so we have to wait until the first call to `update` to set it.
    min_x: f32,
    max_x: f32,
    min_y: f32,
    max_y: f32,
    selected: bool,
    mouse_pos: Vector2<WorldSpace>,
}

impl PaintObject<EguiPainter> for FreehandCurve {
    fn update(&mut self, input: &UserInput, camera: &Camera) {
        if let Some(position) = input.mouse_position() {
            self.mouse_pos = camera.convert_to_world_coordinates(position);
        }
    }
    
    fn draw<'a>(&self, painter: &mut WorldPainter<'a, EguiPainter>, camera: &Camera) {
        if let Some(stroke) = self.stroke {
            for p1p2 in self.points.windows(2) {
                painter.draw_line(p1p2[0], p1p2[1], stroke, camera);
            }
        }
    }
    
    fn is_selected(&self) -> bool {
        self.selected
    }

    fn set_selected(&mut self, value: bool) {
        self.selected = value;
    }

    fn is_under_mouse(&self) -> bool {
        if self.get_bounding_rect().contains_point(self.mouse_pos) {
            for point in self.points.iter() {
                if (*point - self.mouse_pos).length_squared() < 25.0 {
                    return true;
                }
            }
        }
        return false;
    }

    fn get_bounding_rect(&self) -> Rectangle<WorldSpace> {
        Rectangle {
            p1: Vector2::new(self.min_x, self.min_y),
            p2: Vector2::new(self.max_x, self.max_y)
        }
    }

    fn shift_with(&mut self, p: Vector2<WorldSpace>) {
        for point in self.points.iter_mut() {
            *point = *point + p;
        }
        self.min_x += p.x;
        self.min_y += p.y;
        self.max_x += p.x;
        self.max_y += p.y;
    }

    fn resize_to(&mut self, new_size: Rectangle<WorldSpace>) {
        let old_size = self.get_bounding_rect();
        for point in self.points.iter_mut() {
            point.x = new_size.p1.x + (point.x - old_size.p1.x) * (new_size.p2.x - new_size.p1.x) / (old_size.p2.x - old_size.p1.x);
            point.y = new_size.p1.y + (point.y - old_size.p1.y) * (new_size.p2.y - new_size.p1.y) / (old_size.p2.y - old_size.p1.y);
        }

        self.min_x = new_size.p1.x;
        self.min_y = new_size.p1.y;
        self.max_x = new_size.p2.x;
        self.max_y = new_size.p2.y;
    }
}


pub struct FreehandCurveTool {
    curve: FreehandCurve,
    icon: egui::ImageSource<'static>,
}

impl FreehandCurveTool {
    fn new_curve() -> FreehandCurve {
        FreehandCurve {
            stroke: None, 
            points: Vec::new(),
            min_x: f32::INFINITY,
            min_y: f32::INFINITY,
            max_x: f32::NEG_INFINITY,
            max_y: f32::NEG_INFINITY,
            selected: false,
            mouse_pos: Vector2::zero(),
        }
    }
}

impl Default for FreehandCurveTool {
    fn default() -> Self {
        Self {
            curve: Self::new_curve(),
            icon: egui::include_image!("../../img/freehand_tool.png"),
        }
    }
}

impl Tool<EguiPainter, egui::ImageSource<'static>> for FreehandCurveTool {
    fn update(&mut self, input: &UserInput, stroke: Stroke, camera: &Camera) -> Result<Option<Box<dyn PaintObject<EguiPainter>>>, String> {
        self.curve.stroke = Some(stroke);
        if let UserInput::MouseMove { position, button: MouseButton::Left, is_shift_down: false, .. } = input {
            let p = camera.convert_to_world_coordinates(*position);
            let last_point = self.curve.points.last();
            if last_point.is_none() || last_point.is_some_and(|lp| *lp != p) {
                self.curve.points.push(p);
                if p.x < self.curve.min_x {
                    self.curve.min_x = p.x;
                }
                if p.y < self.curve.min_y {
                    self.curve.min_y = p.y;
                }
                if p.x > self.curve.max_x {
                    self.curve.max_x = p.x;
                }
                if p.y > self.curve.max_y {
                    self.curve.max_y = p.y;
                }
            }
        }
        else if self.curve.points.len() > 0 {
            let new_object = std::mem::replace(&mut self.curve, Self::new_curve());
            return Ok(Some(Box::new(new_object)));
        }

        return Ok(None);
    }

    fn draw<'a>(&self, painter: &mut WorldPainter<'a, EguiPainter>, camera: &Camera) {
        self.curve.draw(painter, camera);
    }

    fn display_name(&self) -> &str {
        "free-hand curve"
    }

    fn icon(&self) -> egui::ImageSource<'static> {
        self.icon.clone()
    }
}
