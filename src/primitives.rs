use std::ops::{Add, AddAssign, Mul, Sub};
use std::marker::PhantomData;


pub trait Tag: Clone + Copy {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WorldSpace;
impl Tag for WorldSpace {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ScreenSpace;
impl Tag for ScreenSpace {}


#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct Number<T: Tag> {
    pub value: f32,
    tag: PhantomData<T>,
}

impl<T: Tag> Number<T> {
    fn cast_to<U: Tag>(self) -> Number<U> {
        Number::<U> {
            value: self.value,
            tag: PhantomData,
        }
    }
}

impl<T: Tag> Number<T> {
    pub fn new(value: f32) -> Self {
        Self {
            value,
            tag: PhantomData,
        }
    }
}

impl<T: Tag> Add for Number<T> {
    type Output = Self;
    
    fn add(self, other: Self) -> Self {
        Self::new(self.value + other.value)
    }
}

impl<T: Tag> AddAssign for Number<T> {
    fn add_assign(&mut self, other: Self) {
        self.value += other.value;
    }
}

impl<T: Tag> Sub for Number<T> {
    type Output = Self;
    
    fn sub(self, other: Self) -> Self {
        Self::new(self.value - other.value)
    }
}

impl<T: Tag> Mul<f32> for Number<T> {
    type Output = Self;

    fn mul(self, other: f32) -> Self {
        Self::new(self.value * other)
    }
}

impl<T: Tag> Mul<Number<T>> for Number<T> {
    type Output = Self;

    fn mul(self, other: Number<T>) -> Self {
        Self::new(self.value * other.value)
    }
}



#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector2<T: Tag> {
    pub x: f32,
    pub y: f32,
    tag: PhantomData<T>,
}

impl<T: Tag> Vector2<T> {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y, tag: PhantomData }
    }
    
    pub fn zero() -> Self {
        Self::new(0.0, 0.0)
    }

    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    fn cast_to<U: Tag>(self) -> Vector2<U> {
        Vector2::<U> {
            x: self.x,
            y: self.y,
            tag: PhantomData,
        }
    }
}

impl<T: Tag> Add for Vector2<T> {
    type Output = Self;
    
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            tag: PhantomData,
        }
    }
}

impl<T: Tag> AddAssign for Vector2<T> {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl<T: Tag> Sub for Vector2<T> {
    type Output = Self;
    
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            tag: PhantomData,
        }
    }
}

impl<T: Tag> Mul<f32> for Vector2<T> {
    type Output = Self;

    fn mul(self, other: f32) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
            tag: PhantomData,
        }
    }
}


#[derive(Debug)]
pub struct Camera {
    pub position: Vector2<WorldSpace>,
    pub offset: Vector2<WorldSpace>,
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
    pub fn convert_to_screen_coordinates(&self, point: Vector2<WorldSpace>) -> Vector2<ScreenSpace> {
        ((point - self.position) * self.zoom + self.offset).cast_to::<ScreenSpace>()
    }

    pub fn convert_to_world_coordinates(&self, point: Vector2<ScreenSpace>) -> Vector2<WorldSpace> {
        (point.cast_to::<WorldSpace>() - self.offset) * (1.0 / self.zoom) + self.position
    }

    pub fn distance_to_world_coordinates(&self, distance: Vector2<ScreenSpace>) -> Vector2<WorldSpace> {
        (distance * (1.0 / self.zoom)).cast_to::<WorldSpace>()
    }

    pub fn size_to_world_coordinates(&self, size: Number<ScreenSpace>) -> Number<WorldSpace> {
        (size * (1.0 / self.zoom)).cast_to::<WorldSpace>()
    }

    pub fn size_to_screen_coordinates(&self, size: Number<WorldSpace>) -> Number<ScreenSpace> {
        (size * self.zoom).cast_to::<ScreenSpace>()
    }

    // pub fn stroke_to_world_coordinates(&self, stroke: Stroke<ScreenSpace>) -> Stroke<WorldSpace> {
    //     Stroke {
    //         color: stroke.color,
    //         thickness: self.size_to_world_coordinates(stroke.thickness),
    //     }
    // }

    pub fn stroke_to_screen_coordinates(&self, stroke: Stroke<WorldSpace>) -> Stroke<ScreenSpace> {
        Stroke {
            color: stroke.color,
            thickness: self.size_to_screen_coordinates(stroke.thickness),
        }
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
pub struct Rectangle<T: Tag> {
    pub p1: Vector2<T>,
    pub p2: Vector2<T>,
}

impl<T: Tag> Rectangle<T> {
    pub fn from_point_and_size(p: Vector2<T>, width: Number<T>, height: Number<T>) -> Self {
        Self {
            p1: p,
            p2: p + Vector2::new(width.value, height.value),
        }
    }

    pub fn from_points_well_ordered(p1: Vector2<T>, p2: Vector2<T>) -> Self {
        let x1 = p1.x.min(p2.x);
        let x2 = p1.x.max(p2.x);
        let y1 = p1.y.min(p2.y);
        let y2 = p1.y.max(p2.y);
        Self {
            p1: Vector2::new(x1, y1),
            p2: Vector2::new(x2, y2),
        }
    }

    pub fn contains_point(&self, p: Vector2<T>) -> bool {
        self.p1.x <= p.x && p.x <= self.p2.x &&
        self.p1.y <= p.y && p.y <= self.p2.y
    }

    pub fn shifted_with(self, v: Vector2<T>) -> Self {
        Self {
            p1: self.p1 + v,
            p2: self.p2 + v,
        }
    }

    pub fn vertices(&self) -> [Vector2<T>; 4] {
        [self.p1,
         Vector2::new(self.p2.x, self.p1.y),
         self.p2,
         Vector2::new(self.p1.x, self.p2.y)]
    }

    pub fn vertex_under_point(&self, point: Vector2<T>, radius: Number<T>) -> Option<RectangleVertex> {
        fn is_under_point<U: Tag>(vertex: Vector2<U>, point: Vector2<U>, radius: Number<U>) -> bool {
            (vertex - point).length_squared() < radius.value * radius.value
        }

        use RectangleVertex::*;
        
        if is_under_point(self.p1, point, radius) {
            Some(TopLeft)
        }
        else if is_under_point(self.p2, point, radius) {
            Some(BottomRight)
        }
        else if is_under_point(Vector2::new(self.p2.x, self.p1.y), point, radius) {
            Some(TopRight)
        }
        else if is_under_point(Vector2::new(self.p1.x, self.p2.y), point, radius) {
            Some(BottomLeft)
        }
        else {
            None
        }
    }

    pub fn resize_by_dragging_vertex(&self, vertex: RectangleVertex, drag_delta: Vector2<T>) -> Self {
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
                    p1: Vector2::new(self.p1.x, self.p1.y + drag_delta.y),
                    p2: Vector2::new(self.p2.x + drag_delta.x, self.p2.y),
                }
            },
            BottomLeft => {
                Self {
                    p1: Vector2::new(self.p1.x + drag_delta.x, self.p1.y),
                    p2: Vector2::new(self.p2.x, self.p2.y + drag_delta.y),
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
pub struct Stroke<T: Tag> {
    pub color: Color,
    pub thickness: Number<T>,
}

impl<T: Tag> Stroke<T> {
    pub fn new(color: Color, thickness: Number<T>) -> Self {
        Self {
            color,
            thickness,
        }
    }
}
