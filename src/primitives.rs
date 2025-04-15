use std::ops::{Add, AddAssign, Mul, Sub};


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
    pub position: Vector2,
    pub offset: Vector2,
    pub zoom: f32,
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


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RectangleVertex {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
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

    pub fn vertex_under_point(&self, point: Vector2, radius: f32) -> Option<RectangleVertex> {
        fn is_under_point(vertex: Vector2, point: Vector2, radius: f32) -> bool {
            (vertex - point).length_squared() < radius * radius
        }

        use RectangleVertex::*;
        
        if is_under_point(self.p1, point, radius) {
            Some(TopLeft)
        }
        else if is_under_point(self.p2, point, radius) {
            Some(BottomRight)
        }
        else if is_under_point(Vector2 { x: self.p2.x, y: self.p1.y }, point, radius) {
            Some(TopRight)
        }
        else if is_under_point(Vector2 { x: self.p1.x, y: self.p2.y }, point, radius) {
            Some(BottomLeft)
        }
        else {
            None
        }
    }

    pub fn resize_by_dragging_vertex(&self, vertex: RectangleVertex, drag_delta: Vector2) -> Self {
        use RectangleVertex::*;

        match vertex {
            TopLeft => {
                Self {
                    p1: self.p1 + drag_delta,
                    p2: self.p2,
                }
            },
            TopRight => {
                Self {
                    p1: Vector2 {
                        x: self.p1.x,
                        y: self.p1.y + drag_delta.y,
                    },
                    p2: Vector2 {
                        x: self.p2.x + drag_delta.x,
                        y: self.p2.y,
                    },
                }
            },
            BottomLeft => {
                Self {
                    p1: Vector2 {
                        x: self.p1.x + drag_delta.x,
                        y: self.p1.y,
                    },
                    p2: Vector2 {
                        x: self.p2.x,
                        y: self.p2.y + drag_delta.y,
                    },
                }
            },
            BottomRight => {
                Self {
                    p1: self.p1,
                    p2: self.p2 + drag_delta,
                }
            },
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
    pub color: Color,
    pub thickness: f32,
}

impl Stroke {
    pub fn with_scaled_thickness(self, scale: f32) -> Self {
        Self {
            color: self.color,
            thickness: self.thickness * scale,
        }
    }
}
