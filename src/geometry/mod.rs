use euclid::Point2D;
use euclid::Vector2D;

pub mod debug;
pub mod model;
pub mod nurbs;

#[derive(Clone, Copy, Debug)]
pub struct ModelSpace;

#[derive(Clone, Copy, Debug)]
pub struct CanvasSpace;

#[derive(Clone, Copy, Debug)]
pub struct ScreenSpace;

#[derive(Clone, Copy, Debug)]
pub struct PixelSpace;

#[derive(Clone, Debug)]
pub struct Line<S> {
    pub start: Point2D<f32, S>,
    pub end: Point2D<f32, S>,
}

impl<S> Line<S> {
    pub fn new(start_x: f32, start_y: f32, end_x: f32, end_y: f32) -> Line<S> {
        Line {
            start: Point2D::new(start_x, start_y),
            end: Point2D::new(end_x, end_y),
        }
    }

    pub fn from_tuple((start, end): (Point2D<f32, S>, Point2D<f32, S>)) -> Option<Line<S>> {
        if start.x.is_nan() || start.y.is_nan() || end.x.is_nan() || end.y.is_nan() {
            None
        } else {
            Some(Line { start, end })
        }
    }
}

#[derive(Clone, Debug)]
pub struct Ray<S> {
    pub origin: Point2D<f32, S>,
    pub direction: Vector2D<f32, S>,
}

impl<S> Ray<S> {
    pub fn intersect_line(&self, line: &Line<S>) -> Option<f32> {
        let v1 = self.origin - line.start;
        let s = line.end - line.start;
        let v3 = Vector2D::new(-self.direction.y, self.direction.x);

        let dot = s.dot(v3);
        if dot.abs() < 0.000001 {
            return None;
        }

        let t1 = s.cross(v1) / dot;
        let t2 = v1.dot(v3) / dot;

        if t1 >= 0.0 && (t2 >= 0.0 && t2 <= 1.0) {
            return Some(t1);
        }

        None
    }

    pub fn intersect_lines(&self, lines: impl Iterator<Item = Line<S>>) -> Option<f32> {
        lines
            .filter_map(|l| self.intersect_line(&l))
            .min_by(|a, b| a.partial_cmp(b).unwrap())
    }

    pub fn at(&self, t: f32) -> Point2D<f32, S> {
        self.origin + self.direction * t
    }
}
