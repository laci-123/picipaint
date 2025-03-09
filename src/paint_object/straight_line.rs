use eframe::egui;
use crate::engine::*;


pub struct StraightLine {
    start: Vector2,
    end: Vector2,
    stroke: Stroke,
    selected: bool,
    mouse_pos: Vector2,
}

impl PaintObject<egui::Painter> for StraightLine {
    fn update(&mut self, input: &UserInput, camera: &Camera) {
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
        painter.draw_line(self.start, self.end, self.stroke, camera);
    }
    
    fn is_selected(&self) -> bool {
        self.selected
    }
    
    fn set_selected(&mut self, value: bool) {
        self.selected = value;
    }
    
    fn is_under_mouse(&self) -> bool {
        let epsilon = 10.0;
        (self.start - self.mouse_pos).length() + (self.end - self.mouse_pos).length() < (self.end - self.mouse_pos).length() + epsilon
    }
    
    fn get_bounding_rect(&self) -> Rectangle {
        Rectangle { p1: self.start, p2: self.end }
    }
}


pub struct StraghtLineTool {
    start: Option<Vector2>,
    end: Option<Vector2>,
    stroke: Option<Stroke>, // Only optional because Stroke doesn't have a default value, so we have to wait until the first call to `update` to set it.
    mouse_pos: Vector2,
}

impl StraghtLineTool {
    pub fn new() -> Self{
        Self {
            start: None,
            end: None,
            stroke: None,
            mouse_pos: Vector2::zero(),
        }
    }
}

impl Tool<egui::Painter> for StraghtLineTool {
    fn update(&mut self, input: &UserInput, objects: &mut Vec<Box<dyn PaintObject<egui::Painter>>>, stroke: Stroke, camera: &Camera) {
        self.stroke = Some(stroke);
        
        match input {
            UserInput::MouseMove { position, .. } => {
                self.mouse_pos = *position;
            },
            UserInput::MouseClick { position, button: MouseButton::Left, is_shift_down: false } => {
                if let Some(start) = self.start {
                    if let Some(end) = self.end {
                        let line = StraightLine {
                            start,
                            end,
                            stroke,
                            selected: false,
                            mouse_pos: self.mouse_pos,
                        };
                        objects.push(Box::new(line));
                        self.start = None;
                        self.end = None;
                    }
                    else {
                        self.end = Some(*position);
                    }
                }
                else {
                    self.start = Some(*position);
                }
                self.mouse_pos = *position;
            },
            _ => {
                // do nothing
            },
        }
    }
    
    fn draw<'a>(&self, painter: &mut WorldPainter<'a, egui::Painter>, camera: &Camera) {
        if let Some(stroke) = self.stroke {
            if let Some(start) = self.start {
                if let Some(end) = self.end {
                    painter.draw_line(start, end, stroke, camera);
                }
                else {
                    painter.draw_line(start, self.mouse_pos, stroke, camera);
                }
            }
        }
    }
    
    fn display_name(&self) -> &str {
        "straight line"
    }
}

