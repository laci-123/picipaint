#![allow(unused)]
use std::ops::{Add, AddAssign, Mul, Sub};
use std::sync::Arc;
use image;


#[derive(Clone, Copy, Debug, PartialEq)]
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
    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }
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

impl AddAssign for Vector2 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
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


#[derive(Debug)]
pub struct Camera {
    position: Vector2,
    offset: Vector2,
    zoom: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            position: Vector2::zero(),
            offset: Vector2::zero(),
            zoom: 1.0,
        }
    }
}

impl Camera {
    pub fn convert_to_screen_coordinates(&self, point: Vector2) -> Vector2 {
        (point - self.position) * self.zoom + self.offset
    }

    pub fn convert_to_world_coordinates(&self, point: Vector2) -> Vector2 {
        (point - self.offset) * (1.0 / self.zoom) + self.position
    }
}


#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rectangle {
    pub p1: Vector2,
    pub p2: Vector2,
}

impl Rectangle {
    pub fn from_point_and_size(p: Vector2, width: f32, height: f32) -> Self {
        Self {
            p1: p,
            p2: p + Vector2{ x: width, y: height },
        }
    }

    pub fn from_points_well_ordered(p1: Vector2, p2: Vector2) -> Self {
        let x1 = p1.x.min(p2.x);
        let x2 = p1.x.max(p2.x);
        let y1 = p1.y.min(p2.y);
        let y2 = p1.y.max(p2.y);
        Self {
            p1: Vector2{ x: x1, y: y1 },
            p2: Vector2{ x: x2, y: y2 },
        }
    }

    pub fn contains_point(&self, p: Vector2) -> bool {
        self.p1.x <= p.x && p.x <= self.p2.x &&
        self.p1.y <= p.y && p.y <= self.p2.y
    }

    pub fn shifted_with(self, p: Vector2) -> Self {
        Self {
            p1: self.p1 + p,
            p2: self.p2 + p,
        }
    }

    pub fn vertices(&self) -> [Vector2; 4] {
        [self.p1,
         Vector2 { x: self.p2.x, y: self.p1.y },
         self.p2,
         Vector2 { x: self.p1.x, y: self.p2.y }]
    }

    fn resize_by_dragging_vertex(&self, drag_start: Vector2, drag_delta: Vector2, vertex_radius: f32) -> Option<Self> {
        fn is_dragged(vertex: Vector2, drag_start: Vector2, vertex_radius: f32) -> bool {
            (vertex - drag_start).length_squared() < vertex_radius * vertex_radius
        }

        let new_p1;
        let new_p2;
        if is_dragged(self.p1, drag_start, vertex_radius) {
            // X------o
            // |      |
            // o------o
            new_p1 = self.p1 + drag_delta;
            new_p2 = self.p2;
        }
        else if is_dragged(self.p2, drag_start, vertex_radius) {
            // o------o
            // |      |
            // o------X
            new_p1 = self.p1;
            new_p2 = self.p2 + drag_delta;
        }
        else if is_dragged(Vector2 { x: self.p1.x, y: self.p2.y }, drag_start, vertex_radius) {
            // o------o
            // |      |
            // X------o
            new_p1 = Vector2 { x: self.p1.x + drag_delta.x, y: self.p1.y};
            new_p2 = Vector2 { x: self.p2.x,                y: self.p2.y + drag_delta.y};
        }
        else if is_dragged(Vector2 { x: self.p2.x, y: self.p1.y }, drag_start, vertex_radius) {
            // o------X
            // |      |
            // o------o
            new_p1 = Vector2 { x: self.p1.x,                y: self.p1.y + drag_delta.y};
            new_p2 = Vector2 { x: self.p2.x + drag_delta.x, y: self.p2.y};
        }
        else {
            return None;
        }
        
        Some(Self { p1: new_p1, p2: new_p2 })
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
    pub color: Color,
    pub thickness: f32,
}

impl Stroke {
    fn with_scaled_thickness(self, scale: f32) -> Self {
        Self {
            color: self.color,
            thickness: self.thickness * scale,
        }
    }
}

#[cfg_attr(test, mockall::automock(type Texture = usize;))]
pub trait ScreenPainter {
    type Texture;
    fn draw_line(&mut self, start: Vector2, end: Vector2, stroke: Stroke);
    fn draw_circle(&mut self, center: Vector2, radius: f32, stroke: Stroke);
    fn draw_rectangle(&mut self, rectangle: Rectangle, stroke: Stroke);
    fn draw_rectangle_filled(&mut self, rectangle: Rectangle, color: Color, stroke: Option<Stroke>);
    fn load_image(&mut self, name: &str, image: &image::DynamicImage) -> Self::Texture;
    fn draw_image(&mut self, frame: Rectangle, texture: &Self::Texture);
}


pub struct WorldPainter<'a, P: ScreenPainter> {
    screen_painter: &'a mut P,
}

impl<'a, P: ScreenPainter> WorldPainter<'a, P> {
    pub fn draw_line(&mut self, start: Vector2, end: Vector2, stroke: Stroke, camera: &Camera) {
        let s = camera.convert_to_screen_coordinates(start);
        let e = camera.convert_to_screen_coordinates(end);
        self.screen_painter.draw_line(s, e, stroke.with_scaled_thickness(camera.zoom));
    }
    
    pub fn draw_circle(&mut self, center: Vector2, radius: f32, stroke: Stroke, camera: &Camera) {
        let c = camera.convert_to_screen_coordinates(center);
        let r = camera.zoom * radius;
        self.screen_painter.draw_circle(c, r, stroke.with_scaled_thickness(camera.zoom));
    }
    
    pub fn draw_rectangle(&mut self, rectangle: Rectangle, stroke: Stroke, camera: &Camera) {
        let rect = Rectangle {
            p1: camera.convert_to_screen_coordinates(rectangle.p1),
            p2: camera.convert_to_screen_coordinates(rectangle.p2),
        };
        self.screen_painter.draw_rectangle(rect, stroke.with_scaled_thickness(camera.zoom));
    }
    
    pub fn draw_rectangle_filled(&mut self, rectangle: Rectangle, color: Color, stroke: Option<Stroke>, camera: &Camera) {
        let rect = Rectangle {
            p1: camera.convert_to_screen_coordinates(rectangle.p1),
            p2: camera.convert_to_screen_coordinates(rectangle.p2),
        };
        self.screen_painter.draw_rectangle_filled(rect, color, stroke.map(|s| s.with_scaled_thickness(camera.zoom)));
    }

    pub fn load_image(&mut self, name: &str, image: &image::DynamicImage) -> P::Texture {
        self.screen_painter.load_image(name, image)
    }
    
    pub fn draw_image(&mut self, frame: Rectangle, texture: &P::Texture, camera: &Camera) {
        let rect = Rectangle {
            p1: camera.convert_to_screen_coordinates(frame.p1),
            p2: camera.convert_to_screen_coordinates(frame.p2),
        };
        self.screen_painter.draw_image(rect, texture);
    }
}


#[derive(PartialEq, Debug)]
pub enum MouseButton {
    None,
    Left,
    Middle,
    Right,
}


#[derive(PartialEq, Debug)]
pub enum UserInput {
    Nothing,
    MouseClick {
        position: Vector2,
        button: MouseButton,
        is_shift_down: bool,
    },
    MouseMove {
        position: Vector2,
        delta: Vector2,
        button: MouseButton,
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
    Delete,
}

impl UserInput {
    pub fn mouse_position(&self) -> Option<Vector2> {
        match self {
            Self::MouseClick { position, .. } => Some(*position),
            Self::MouseMove { position, .. }  => Some(*position),
            _                                 => None,
        }
    }

    pub fn mouse_delta(&self) -> Option<Vector2> {
        match self {
            Self::MouseMove { delta, .. } => Some(*delta),
            _                             => None,
        }
    }

    pub fn mouse_is_up(&self) -> bool {
        match self {
            Self::MouseMove { button: MouseButton::None, .. } => true,
            Self::MouseMove { .. }                            => false,
            Self::MouseClick { .. }                           => false,
            _                                                 => true,
        }
    }
}

pub trait PaintObject<P: ScreenPainter> {
    fn update(&mut self, input: &UserInput, camera: &Camera);
    fn draw<'a>(&self, painter: &mut WorldPainter<'a, P>, camera: &Camera);
    fn is_selected(&self) -> bool;
    fn set_selected(&mut self, value: bool);
    fn is_under_mouse(&self) -> bool;
    fn get_bounding_rect(&self) -> Rectangle;
    fn shift_with(&mut self, p: Vector2);
    fn resize_to(&mut self, new_size: Rectangle);
}


#[cfg_attr(test, mockall::automock)]
pub trait Tool<P: ScreenPainter, IconType> {
    fn update(&mut self, input: &UserInput, stroke: Stroke, camera: &Camera) -> Result<Option<Box<dyn PaintObject<P>>>, String>;
    fn draw<'a>(&self, painter: &mut WorldPainter<'a, P>, camera: &Camera);
    fn display_name(&self) -> &str;
    fn icon(&self) -> IconType;
}


pub struct ToolIterator<'a, P: ScreenPainter, IconType> {
    tools: &'a Vec<Box<dyn Tool<P, IconType>>>,
    index: usize,
}

impl<'a, P: ScreenPainter, IconType> Iterator for ToolIterator<'a, P, IconType> {
    type Item = &'a dyn Tool<P, IconType>;

    fn next(&mut self) -> Option<&'a dyn Tool<P, IconType>> {
        if let Some(tool) = self.tools.get(self.index) {
            self.index += 1;
            Some(tool.as_ref())
        }
        else {
            None
        }
    }
}


pub struct Engine<P: ScreenPainter, IconType> {
    objects: Vec<Box<dyn PaintObject<P>>>,
    tools: Vec<Box<dyn Tool<P, IconType>>>,
    to_be_deleted: Vec<usize>,
    selected_tool_index: Option<usize>,
    view_width: f32,
    view_height: f32,
    camera: Camera,
    background_color: Color,
    objects_are_dragged: bool,
}

impl<P: ScreenPainter, IconType> Engine<P, IconType> {
    pub fn new(tools: Vec<Box<dyn Tool<P, IconType>>>) -> Self {
        Self {
            objects: Vec::new(),
            tools,
            to_be_deleted: Vec::new(),
            selected_tool_index: None,
            view_width: 0.0,
            view_height: 0.0,
            camera: Camera::default(),
            background_color: Color::from_rgb(0, 0, 0),
            objects_are_dragged: false,
        }
    }

    pub fn add_object(&mut self, object: impl PaintObject<P> + 'static) {
        self.objects.push(Box::new(object));
    }
    
    pub fn update(&mut self, input: UserInput, stroke: Stroke, background_color: Color, view_width: f32, view_height: f32) -> Result<(), String> {
        self.background_color = background_color;
        self.view_width = view_width;
        self.view_height = view_height;
        self.camera.offset = Vector2 { x: view_width / 2.0, y: view_height / 2.0 };

        match input {
            UserInput::Pan { delta } => {
                self.camera.position = self.camera.position + delta * (1.0 / self.camera.zoom);
            },
            UserInput::Zoom { delta } => {
                self.camera.zoom += delta;
                if self.camera.zoom < 0.0 {
                    self.camera.zoom = 0.0;
                }
            },
            _ => {
                self.update_tools_and_objects(input, stroke)?;
            },
        }

        Ok(())
    }

    fn update_tools_and_objects(&mut self, input: UserInput, stroke: Stroke) -> Result<(), String> {
        if let Some(tool_index) = self.selected_tool_index {
            if let Some(tool) = self.tools.get_mut(tool_index) {
                if let Some(new_object) = tool.update(&input, stroke, &self.camera)? {
                    self.objects.push(new_object);
                }
            }
        }

        if input == UserInput::SelectAll {
            self.select_tool(None);
        }
        if input.mouse_is_up() {
            self.objects_are_dragged = false;
        }

        for (i, object) in self.objects.iter_mut().enumerate() {
            object.update(&input, &self.camera);

            if self.selected_tool_index.is_none() {
                if input == UserInput::SelectAll {
                    object.set_selected(true);
                    continue;
                }
                if input == UserInput::DeselectAll {
                    object.set_selected(false);
                    continue;
                }
                if input == UserInput::Delete && object.is_selected() {
                    self.to_be_deleted.push(i);
                    continue;
                }

                let left_click    = matches!(input, UserInput::MouseClick { button: MouseButton::Left, .. });
                let shift_is_down = matches!(input, UserInput::MouseClick { is_shift_down: true, .. });

                if left_click {
                    if object.is_under_mouse() {
                        if shift_is_down {
                            object.set_selected(!object.is_selected());
                        }
                        else {
                            object.set_selected(true);
                        }
                    }
                    else {
                        if !shift_is_down {
                            object.set_selected(false);
                        }
                    }
                }
                if object.is_selected() && object.is_under_mouse() {
                    self.objects_are_dragged = true;
                }
            }
        }

        for object in self.objects.iter_mut() {
            if let (Some(delta), Some(position)) = (input.mouse_delta(), input.mouse_position()) {
                if object.is_selected() {
                    if let Some(new_size) = object.get_bounding_rect().resize_by_dragging_vertex(self.camera.convert_to_world_coordinates(position), delta, 10.0) {
                        object.resize_to(new_size);
                    }
                    else if self.objects_are_dragged {
                        object.shift_with(delta * (1.0 / self.camera.zoom));
                    }
                }
            }
        }

        for i in self.to_be_deleted.iter().rev() {
            // going in reverse order to avoid shifting indeces
            self.objects.swap_remove(*i);
        }
        self.to_be_deleted.clear();

        Ok(())
    }

    pub fn draw(&self, screen_painter: &mut P) {
        screen_painter.draw_rectangle_filled(Rectangle::from_point_and_size(Vector2::zero(), self.view_width, self.view_height), self.background_color, None);

        for object in self.objects.iter() {
            let mut world_painter = WorldPainter { screen_painter };
            object.draw(&mut world_painter, &self.camera);
            if object.is_selected() {
                // Not using world painter so that the thickness of the selection markers
                // won't change with zoom level.
                let world_rect = object.get_bounding_rect();
                let screen_rect = Rectangle {
                    p1: self.camera.convert_to_screen_coordinates(world_rect.p1),
                    p2: self.camera.convert_to_screen_coordinates(world_rect.p2),
                };
                let selection_marker_color = Color::from_rgb(255, 255, 255);
                screen_painter.draw_rectangle(screen_rect, Stroke { color: selection_marker_color, thickness: 1.0 });
                for vertex in screen_rect.vertices() {
                    screen_painter.draw_circle(vertex, 5.0, Stroke { color: selection_marker_color, thickness: 1.0 });
                }
            }
        }

        if let Some(tool_index) = self.selected_tool_index {
            let mut world_painter = WorldPainter { screen_painter };
            if let Some(tool) = self.tools.get(tool_index) {
                tool.draw(&mut world_painter, &self.camera);
            }
        }
    }

    pub fn tools_iter(&self) -> ToolIterator<P, IconType> {
        ToolIterator { tools: &self.tools, index: 0 }
    }

    pub fn select_tool(&mut self, index: Option<usize>) {
        self.selected_tool_index = index;
        if let Some(i) = index {
            let current_tool = &mut self.tools[i];
            for object in self.objects.iter_mut() {
                object.set_selected(false);
            }
        }
    }

    pub fn get_selected_tool_index(&self) -> Option<usize> {
        self.selected_tool_index
    }
}


#[cfg(test)]
mod tests;
