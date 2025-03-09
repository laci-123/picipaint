use crate::engine::*;
use eframe::egui;


pub struct FreehandCurve {
    points: Vec<Vector2>,
    stroke: Option<Stroke>, // Only optional because Stroke doesn't have a default value, so we have to wait until the first call to `update` to set it.
    min_x: f32,
    max_x: f32,
    min_y: f32,
    max_y: f32,
    selected: bool,
    mouse_pos: Vector2,
}

impl PaintObject<egui::Painter> for FreehandCurve {
    fn update(&mut self, input: &UserInput) {
        match input {
            UserInput::MouseMove { position, .. } => {
                self.mouse_pos = *position;
            },
            UserInput::MouseClick { position, .. } => {
                self.mouse_pos = *position;
            },
            _ => {
                // do nothing
            },
        }
    }
    
    fn draw<'a>(&self, painter: &mut WorldPainter<'a, egui::Painter>, camera: &Camera) {
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

    fn get_bounding_rect(&self) -> Rectangle {
        Rectangle {
            p1: Vector2 { x: self.min_x, y: self.min_y },
            p2: Vector2 { x: self.max_x, y: self.max_y }
        }
    }
}


pub struct FreehandCurveTool {
    curve: FreehandCurve,
}

impl FreehandCurveTool {
    pub fn new() -> Self {
        Self {
            curve: Self::new_curve(),
        }
    }

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

impl Tool<egui::Painter> for FreehandCurveTool {
    fn update(&mut self, input: &UserInput, objects: &mut Vec<Box<dyn PaintObject<egui::Painter>>>, stroke: Stroke) {
        self.curve.stroke = Some(stroke);
        if let UserInput::MouseMove { position, button: MouseButton::Left, is_shift_down: false } = input {
            let last_point = self.curve.points.last();
            if last_point.is_none() || last_point.is_some_and(|lp| lp != position) {
                self.curve.points.push(*position);
                if position.x < self.curve.min_x {
                    self.curve.min_x = position.x;
                }
                if position.y < self.curve.min_y {
                    self.curve.min_y = position.y;
                }
                if position.x > self.curve.max_x {
                    self.curve.max_x = position.x;
                }
                if position.y > self.curve.max_y {
                    self.curve.max_y = position.y;
                }
            }
        }
        else if self.curve.points.len() > 0 {
            let new_object = std::mem::replace(&mut self.curve, Self::new_curve());
            objects.push(Box::new(new_object));
        }
    }

    fn draw<'a>(&self, painter: &mut WorldPainter<'a, egui::Painter>, camera: &Camera) {
        self.curve.draw(painter, camera);
    }

    fn display_name(&self) -> &str {
        "free-hand curve"
    }
}
