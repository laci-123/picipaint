#![allow(unused)]
use std::ops::{Add, Mul, Sub};


#[derive(Clone, Copy, Debug)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
        }
    }
}

impl Vector2 {
    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

impl Add for Vector2 {
    type Output = Self;
    
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Vector2 {
    type Output = Self;
    
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<f32> for Vector2 {
    type Output = Self;

    fn mul(self, other: f32) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
        }
    }
}


pub struct Camera {
    position: Vector2,
    zoom: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            position: Vector2::zero(),
            zoom: 1.0,
        }
    }
}

impl Camera {
    pub fn convert_to_screen_coordinates(&self, point: Vector2) -> Vector2 {
        (point - self.position) * self.zoom
    }

    pub fn convert_to_world_coordinates(&self, point: Vector2) -> Vector2 {
        point * (1.0 / self.zoom) + self.position
    }
}


#[derive(Clone, Copy)]
pub struct Rectangle {
    pub p1: Vector2,
    pub p2: Vector2,
}

impl Rectangle {
    fn from_point_and_size(p: Vector2, width: f32, height: f32) -> Self {
        Self {
            p1: p,
            p2: p + Vector2{ x: width, y: height },
        }
    }
}


#[derive(Clone, Copy)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

impl Color {
    pub fn from_rgb(red: u8, green: u8, blue: u8) -> Self {
        Self {
            red,
            green,
            blue,
            alpha: 255,
        }
    }
}


#[derive(Clone, Copy)]
pub struct Stroke {
    color: Color,
    thickness: f32,
}


pub trait ScreenPainter {
    fn draw_line(&mut self, start: Vector2, end: Vector2, stroke: Stroke);
    fn draw_circle(&mut self, center: Vector2, radius: f32, stroke: Stroke);
    fn draw_rectangle(&mut self, rectangle: Rectangle, stroke: Stroke);
    fn draw_rectangle_filled(&mut self, rectangle: Rectangle, color: Color, stroke: Option<Stroke>);
}


pub struct WorldPainter<'a, P: ScreenPainter> {
    screen_painter: &'a mut P,
}

impl<'a, P: ScreenPainter> WorldPainter<'a, P> {
    fn draw_line(&mut self, start: Vector2, end: Vector2, stroke: Stroke, camera: &Camera) {
        let s = camera.convert_to_screen_coordinates(start);
        let e = camera.convert_to_screen_coordinates(end);
        self.screen_painter.draw_line(s, e, stroke);
    }
    
    fn draw_circle(&mut self, center: Vector2, radius: f32, stroke: Stroke, camera: &Camera) {
        let c = camera.convert_to_screen_coordinates(center);
        let r = camera.zoom * radius;
        self.screen_painter.draw_circle(c, r, stroke);
    }
    
    fn draw_rectangle(&mut self, rectangle: Rectangle, stroke: Stroke, camera: &Camera) {
        let rect = Rectangle {
            p1: camera.convert_to_screen_coordinates(rectangle.p1),
            p2: camera.convert_to_screen_coordinates(rectangle.p2),
        };
        self.screen_painter.draw_rectangle(rect, stroke);
    }
    
    fn draw_rectangle_filled(&mut self, rectangle: Rectangle, color: Color, stroke: Option<Stroke>, camera: &Camera) {
        let rect = Rectangle {
            p1: camera.convert_to_screen_coordinates(rectangle.p1),
            p2: camera.convert_to_screen_coordinates(rectangle.p2),
        };
        self.screen_painter.draw_rectangle_filled(rect, color, stroke);
    }
}


pub enum MouseButton {
    Left,
    Middle,
    Right,
}


pub enum UserInput {
    Nothing,
    MouseClick {
        position: Vector2,
        button: MouseButton,
        is_shift_down: bool,
    },
    MouseMove {
        position: Vector2,
        is_shift_down: bool,
    },
    SelectAll,
    DeselectAll,
    Zoom {
        delta: f32,
    },
    Pan {
        delta: Vector2,
    },
    Resize {
        new_width: f32,
        new_height: f32,
    }
}


pub trait PaintObject<P: ScreenPainter> {
    fn update(&mut self, input: &UserInput, camera: &Camera);
    fn draw(&self, painter: &mut WorldPainter<P>, camera: &Camera);
    fn is_selected(&self) -> bool;
    fn set_selected(&mut self, value: bool);
    fn is_under_mouse(&self) -> bool;
    fn get_bounding_rect(&self) -> Rectangle;
}


pub trait Tool<P: ScreenPainter> {
    fn update(&mut self, input: &UserInput, objects: &mut Vec<Box<dyn PaintObject<P>>>, stroke: Stroke, camera: &Camera);
    fn draw(&self, painter: &mut WorldPainter<P>, camera: &Camera);
    fn before_deactivate(&mut self, objects: &mut Vec<Box<dyn PaintObject<P>>>);
    fn display_name(&self) -> &str;
}


pub struct ToolIterator<'a, P: ScreenPainter> {
    tools: &'a Vec<Box<dyn Tool<P>>>,
    index: usize,
}

impl<'a, P: ScreenPainter> Iterator for ToolIterator<'a, P> {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        if let Some(tool) = self.tools.get(self.index) {
            let name = String::from(tool.display_name());
            self.index += 1;
            Some(name)
        }
        else {
            None
        }
    }
}


pub struct Engine<P: ScreenPainter> {
    objects: Vec<Box<dyn PaintObject<P>>>,
    tools: Vec<Box<dyn Tool<P>>>,
    selected_tool_index: usize,
    view_width: f32,
    view_height: f32,
    camera: Camera,
    background_color: Color,
}

impl<P: ScreenPainter> Engine<P> {
    pub fn new(tools: Vec<Box<dyn Tool<P>>>, view_width: f32, view_height: f32) -> Self {
        Self {
            objects: Vec::new(),
            tools,
            selected_tool_index: 0,
            view_width,
            view_height,
            camera: Camera::default(),
            background_color: Color::from_rgb(0, 0, 0),
        }
    }
    
    pub fn update(&mut self, input: UserInput, stroke: Stroke, background_color: Color) {
        self.background_color = background_color;

        match input {
            UserInput::Pan { delta } => {
                self.camera.position = self.camera.position + self.camera.convert_to_world_coordinates(delta);
            },
            UserInput::Zoom { delta } => {
                self.camera.zoom += delta;
            },
            UserInput::Resize { new_width, new_height } => {
                self.view_width = new_width;
                self.view_height = new_height;
            },
            _ => {
                self.update_tools_and_objects(input, stroke);
            },
        }
    }

    fn update_tools_and_objects(&mut self, input: UserInput, stroke: Stroke) {
        for tool in self.tools.iter_mut() {
            tool.update(&input, &mut self.objects, stroke, &self.camera);
        }

        for object in self.objects.iter_mut() {
            object.update(&input, &self.camera);
        }
    }

    pub fn draw(&self, screen_painter: &mut P) {
        screen_painter.draw_rectangle_filled(Rectangle::from_point_and_size(Vector2::zero(), self.view_width, self.view_height), self.background_color, None);

        let mut world_painter = WorldPainter { screen_painter };
        
        for object in self.objects.iter() {
            object.draw(&mut world_painter, &self.camera);
        }

        for tool in self.tools.iter() {
            tool.draw(&mut world_painter, &self.camera);
        }
    }

    pub fn tools_iter(&self) -> ToolIterator<P> {
        ToolIterator { tools: &self.tools, index: 0 }
    }

    pub fn select_tool(&mut self, index: usize) {
        let current_tool = &mut self.tools[self.selected_tool_index];
        current_tool.before_deactivate(&mut self.objects);
        self.selected_tool_index = index;
    }
}


#[cfg(test)]
mod tests;
