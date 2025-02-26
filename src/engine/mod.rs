use std::ops::{Add, Mul};


#[derive(Clone, Copy)]
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

impl Add for Vector2 {
    type Output = Self;
    
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
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


pub trait Painter {
    fn draw_line(&self, start: Vector2, end: Vector2, stroke: Stroke);
    fn draw_circle(&self, center: Vector2, radius: f32, stroke: Stroke);
    fn draw_rectangle(&self, rectangle: Rectangle, stroke: Stroke);
    fn draw_rectangle_filled(&self, rectangle: Rectangle, color: Color, stroke: Option<Stroke>);
}


pub enum MouseButton {
    Left,
    Middle,
    Right,
}


pub enum UserInput {
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

impl UserInput {
    fn map_position<F: FnOnce(Vector2) -> Vector2>(self, f: F) -> Self {
        match self {
            Self::MouseClick { position, button, is_shift_down } => Self::MouseClick { position: f(position), button, is_shift_down },
            Self::MouseMove  { position, is_shift_down }         => Self::MouseMove  { position: f(position), is_shift_down },
            other => other,
        }
    }
}


pub trait PaintObject<P: Painter> {
    fn update(&mut self, input: &UserInput);
    fn draw(&self, painter: &P);
    fn is_selected(&self) -> bool;
    fn set_selected(&mut self, value: bool);
    fn is_under_mouse(&self) -> bool;
    fn get_bounding_rect(&self) -> Rectangle;
}


pub trait Tool<P: Painter> {
    fn update(&mut self, input: &UserInput, objects: &mut Vec<Box<dyn PaintObject<P>>>, stroke: Stroke);
    fn draw(&self, painter: &P);
    fn before_deactivate(&mut self, objects: &mut Vec<Box<dyn PaintObject<P>>>);
    fn display_name(&self) -> &str;
}


pub struct ToolIterator<'a, P: Painter> {
    tools: &'a Vec<Box<dyn Tool<P>>>,
    index: usize,
}

impl<'a, P: Painter> Iterator for ToolIterator<'a, P> {
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


pub struct Engine<P: Painter> {
    objects: Vec<Box<dyn PaintObject<P>>>,
    tools: Vec<Box<dyn Tool<P>>>,
    selected_tool_index: usize,
    view_width: f32,
    view_height: f32,
    camera_position: Vector2,
    camera_zoom: f32,
    background_color: Color,
}

impl<P: Painter> Engine<P> {
    pub fn new(tools: Vec<Box<dyn Tool<P>>>, view_width: f32, view_height: f32) -> Self {
        Self {
            objects: Vec::new(),
            tools,
            selected_tool_index: 0,
            view_width,
            view_height,
            camera_position: Vector2{ x: view_width / 2.0, y: view_height / 2.0 },
            camera_zoom: 1.0,
            background_color: Color::from_rgb(0, 0, 0),
        }
    }
    
    pub fn update(&mut self, input: UserInput, stroke: Stroke, background_color: Color) {
        self.background_color = background_color;

        match input {
            UserInput::Pan { delta } => {
                self.camera_position = self.camera_position + delta;
            },
            UserInput::Zoom { delta } => {
                self.camera_zoom += delta;
            },
            UserInput::Resize { new_width, new_height } => {
                self.view_width = new_width;
                self.view_height = new_height;
            },
            _ => {
                // TODO: transform input based on camera position and zoom
                self.update_tools_and_objects(input, stroke);
            },
        }
    }

    fn update_tools_and_objects(&mut self, input: UserInput, stroke: Stroke) {
        for tool in self.tools.iter_mut() {
            tool.update(&input, &mut self.objects, stroke);
        }

        for object in self.objects.iter_mut() {
            object.update(&input);
        }
    }

    pub fn draw(&self, painter: &P) {
        painter.draw_rectangle_filled(Rectangle::from_point_and_size(Vector2::zero(), self.view_width, self.view_height), self.background_color, None);
        
        for object in self.objects.iter() {
            object.draw(painter);
        }

        for tool in self.tools.iter() {
            tool.draw(painter);
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
