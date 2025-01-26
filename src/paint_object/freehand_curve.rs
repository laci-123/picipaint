use super::*;


pub struct FreehandCurve {
    points: Vec<Vec2>,
    color: Color,
    thickness: f32,
    min_x: f32,
    max_x: f32,
    min_y: f32,
    max_y: f32,
    selected: bool,
}

impl PaintObject for FreehandCurve {
    fn draw(&self) {
        for p1p2 in self.points.windows(2) {
            let [p1, p2] = p1p2 else {unreachable!()};
            draw_line(p1.x, p1.y, p2.x, p2.y, self.thickness, self.color);
        }
    }

    fn is_selected(&self) -> bool {
        self.selected
    }
    
    fn set_selected(&mut self, value: bool) {
        self.selected = value;
    }
    
    fn is_under_mouse(&self, mouse_pos: Vec2) -> bool {
        let in_bounding_rect = self.min_x < mouse_pos.x && mouse_pos.x < self.max_x &&
                               self.min_y < mouse_pos.y && mouse_pos.y < self.max_y;
        if in_bounding_rect {
            for point in self.points.iter() {
                if (*point - mouse_pos).length_squared() < 10.0 {
                    return true;
                }
            }
        }
        return false;
    }
}


pub struct FreehandCurveMaker {
    curve: FreehandCurve,
}

impl FreehandCurveMaker {
    pub fn new(color: Color, thickness: f32) -> Self {
        Self {
            curve: Self::new_curve(color, thickness),
        }
    }

    fn new_curve(color: Color, thickness: f32) -> FreehandCurve {
        FreehandCurve {
            color,
            thickness,
            points: Vec::new(),
            min_x: f32::INFINITY,
            min_y: f32::INFINITY,
            max_x: f32::NEG_INFINITY,
            max_y: f32::NEG_INFINITY,
            selected: false,
        }
    }
}

impl PaintObjectMaker<FreehandCurve> for FreehandCurveMaker {
    fn update_and_draw(&mut self, mouse_pos: Vec2) -> Option<FreehandCurve> {
        if is_mouse_button_down(MouseButton::Left) {
            self.curve.min_x = self.curve.min_x.min(mouse_pos.x);
            self.curve.min_y = self.curve.min_y.min(mouse_pos.y);
            self.curve.max_x = self.curve.max_x.max(mouse_pos.x);
            self.curve.max_y = self.curve.max_y.max(mouse_pos.y);
            self.curve.points.push(mouse_pos);
            self.curve.draw();
            None
        }
        else if self.curve.points.len() > 0 {
            let new_curve = Self::new_curve(self.curve.color, self.curve.thickness);
            Some(std::mem::replace(&mut self.curve, new_curve))
        }
        else {
            None
        }
    }
}
