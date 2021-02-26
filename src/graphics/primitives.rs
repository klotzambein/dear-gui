use euclid::Point2D;
use glium::implement_vertex;
use glium::vertex::{Attribute as GLAttribute, AttributeType as GLAttributeType};

use crate::geometry::Line as GLine;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vf2(f32, f32);

impl Vf2 {
    pub fn new(x: f32, y: f32) -> Vf2 {
        Vf2(x, y)
    }
}

unsafe impl GLAttribute for Vf2 {
    /// Get the type of data.
    fn get_type() -> GLAttributeType {
        GLAttributeType::F32F32
    }
}

impl<T> From<Point2D<f32, T>> for Vf2 {
    fn from(p: Point2D<f32, T>) -> Self {
        Vf2::new(p.x, p.y)
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color(f32, f32, f32, f32);

unsafe impl GLAttribute for Color {
    /// Get the type of data.
    fn get_type() -> GLAttributeType {
        GLAttributeType::F32F32F32F32
    }
}

impl Color {
    pub const BLACK: Color = Color(0.0, 0.0, 0.0, 1.0);
    pub const WHITE: Color = Color(1.0, 1.0, 1.0, 1.0);
    pub const BLUE: Color = Color(0.1, 0.1, 0.9, 1.0);
    pub const RED: Color = Color(0.9, 0.1, 0.1, 1.0);
    pub const GREEN: Color = Color(0.1, 0.9, 0.1, 1.0);

    pub fn to_rgb_array(self) -> [f32; 3] {
        [self.0, self.1, self.2]
    }
    pub fn to_rgba_array(self) -> [f32; 4] {
        [self.0, self.1, self.2, self.3]
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LinePoint {
    vertex: Vf2,
}
implement_vertex!(LinePoint, vertex);

impl LinePoint {
    pub fn from_point<U>(vec: Point2D<f32, U>) -> LinePoint {
        LinePoint {
            vertex: Vf2::new(vec.x, vec.y),
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ColoredPoint {
    pub vertex: Vf2,
    pub color: Color,
}
implement_vertex!(ColoredPoint, vertex, color);

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Line {
    start: Vf2,
    end: Vf2,
}
implement_vertex!(Line, start, end);

impl Line {
    pub fn from_points<U>(start: Point2D<f32, U>, end: Point2D<f32, U>) -> Line {
        Line {
            start: Vf2::new(start.x, start.y),
            end: Vf2::new(end.x, end.y),
        }
    }
    pub fn from_line<U>(line: GLine<U>) -> Line {
        Line::from_points(line.start, line.end)
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ColoredLine {
    pub start: Vf2,
    pub end: Vf2,
    pub color: Color,
}
implement_vertex!(ColoredLine, start, end, color);

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sprite {
    pub vertex: Vf2,
    pub size: Vf2,
    pub texture_index: i32,
}
implement_vertex!(Sprite, vertex, size, texture_index);
