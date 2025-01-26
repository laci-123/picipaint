use super::{PaintObject, PaintObjectMaker};
use macroquad::prelude::*;


pub struct StraightLine {
    start: Vec2,
    end: Vec2,
    color: Color,
    thickness: f32,
    selected: bool,
}

impl PaintObject for StraightLine {
    fn draw(&self) {
        draw_line(self.start.x, self.start.y, self.end.x, self.end.y, self.thickness, self.color);
    }

    fn is_selected(&self) -> bool {
        self.selected
    }

    fn set_selected(&mut self, value: bool) {
        self.selected = value;
    }

    fn is_under_mouse(&self, mouse_pos: Vec2) -> bool {
        let length           = (self.end - self.start).length();
        let mouse_from_start = (mouse_pos - self.start).length();
        let mouse_from_end   = (mouse_pos - self.end).length();

        mouse_from_start + mouse_from_end < length + 10.0
    }
}


pub struct StraightLineMaker {
    start: Option<Vec2>,
    color: Color,
    thickness: f32,
}

impl StraightLineMaker {
    pub fn new(color: Color, thickness: f32) -> Self{
        Self {
            start: None,
            color,
            thickness,
        }
    }
}

impl PaintObjectMaker<StraightLine> for StraightLineMaker {
    fn update_and_draw(&mut self, mouse_pos: Vec2) -> Option<StraightLine> {
        if is_mouse_button_down(MouseButton::Left) {
            match self.start {
                None => {
                    self.start = Some(mouse_pos);
                },
                Some(start) => {
                    draw_line(start.x, start.y, mouse_pos.x, mouse_pos.y, self.thickness, self.color);
                },
            }
        }
        else if let Some(start) = self.start {
            return Some(StraightLine {
                start,
                end: mouse_pos,
                color: self.color,
                thickness: self.thickness,
                selected: false
            })
        }
        return None;
    }

    fn reset(&mut self) {
        self.start = None;
    }
}
