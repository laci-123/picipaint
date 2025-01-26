use macroquad::prelude::*;


pub trait PaintObject {
    fn draw(&self);
    fn is_selected(&self) -> bool;
    fn set_selected(&mut self, value: bool);
    fn is_under_mouse(&self, mouse_in_world: Vec2) -> bool;
}


pub trait PaintObjectMaker<PO> {
    fn update_and_draw(&mut self, mouse_in_world: Vec2) -> Option<PO>;
    fn reset(&mut self);
}


pub mod straight_line;
