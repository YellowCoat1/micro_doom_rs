use ggez::{Context, graphics::{Canvas, Color, Mesh}, mint::Point2};
use std::ops::{Add, Sub, Mul};

use super::vecs::Vec2;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LineSegment {
    pub start: Vec2,
    pub end: Vec2,
}

impl From<LineSegment> for (Vec2, Vec2) {
    fn from(line: LineSegment) -> Self {
        (line.start, line.end)
    }
}

impl From<(Vec2, Vec2)> for LineSegment {
    fn from(points: (Vec2, Vec2)) -> Self {
        Self {
            start: points.0,
            end: points.1,
        }
    }
}

impl From<(f32, f32, f32, f32)> for LineSegment {
    fn from(coords: (f32, f32, f32, f32)) -> Self {
        Self {
            start: Vec2::new(coords.0, coords.1),
            end: Vec2::new(coords.2, coords.3),
        }
    }
}

impl From<LineSegment> for (f32, f32, f32, f32) {
    fn from(line: LineSegment) -> Self {
        (line.start.x, line.start.y, line.end.x, line.end.y)
    }
}

impl Add for LineSegment {
    type Output = LineSegment;

    fn add(self, other: LineSegment) -> LineSegment {
        LineSegment {
            start: self.start + other.start,
            end: self.end + other.end,
        }
    }
}

impl Sub for LineSegment {
    type Output = LineSegment;

    fn sub(self, other: LineSegment) -> LineSegment {
        LineSegment {
            start: self.start - other.start,
            end: self.end - other.end,
        }
    }
}

impl Add<f32> for LineSegment {
    type Output = LineSegment;

    fn add(self, scalar: f32) -> LineSegment {
        LineSegment {
            start: self.start + Vec2::new(scalar, scalar),
            end: self.end + Vec2::new(scalar, scalar),
        }
    }
}

impl Mul<f32> for LineSegment {
    type Output = LineSegment;

    fn mul(self, scalar: f32) -> LineSegment {
        LineSegment {
            start: self.start * scalar,
            end: self.end * scalar,
        }
    }
}


impl LineSegment {
    pub fn draw(&self, ctx: &mut Context, canvas: &mut Canvas, color: Color) {
        let point1 = Point2 {
            x: self.start.x,
            y: self.start.y,
        };
        let point2 = Point2 {
            x: self.end.x,
            y: self.end.y,
        };
        let line = Mesh::new_line(
            ctx,
            &[point1, point2],
            1.5,
            color
        ).unwrap();
        canvas.draw(&line, ggez::graphics::DrawParam::default())
    }
}


impl LineSegment {
    pub fn new(start: Vec2, end: Vec2) -> Self {
        Self { start, end }
    }

    pub fn length(&self) -> f32 {
        (self.end - self.start).length()
    }

    pub fn midpoint(&self) -> Vec2 {
        (self.start + self.end) * 0.5
    }
}


/// Finds the intersection point of two lines. Note that this is lines, NOT line segments, so the
/// point may lie outside the segments. If the lines are parallel, returns None.
fn intersection_point(a: &LineSegment, b: &LineSegment) -> Option<Vec2> {
    let denom = (a.start.x - a.end.x) * (b.start.y - b.end.y) - (a.start.y - a.end.y) * (b.start.x - b.end.x);
    if denom == 0.0 {
        return None
    }
    let top = (a.start.x - b.start.x)*(b.start.y - b.end.y) - (a.start.y - b.start.y)*(b.start.x - b.end.x);

    let calced = top/denom;

    Some(a.start+(a.end-a.start) * calced)
}

/// Treats the first line as a ray and the second as a segment. Returns the intersection
/// point if it lies on the segment, None otherwise.
pub fn intersection_point_segment(a: &LineSegment, b: &LineSegment) -> Option<Vec2> {
    let p = intersection_point(a, b)?;

    // Check x bounds for segment b
    if p.x < b.start.x.min(b.end.x) || p.x > b.start.x.max(b.end.x) {
       return None;
    }

    if a.start.x > a.end.x {
        if p.x > a.start.x {
            return None
        }
    } else if a.start.x < a.end.x {
        if p.x < a.start.x {
            return None
        }
    } else {
        if p.x != a.start.x {
            return None
        }
    }


    Some(p)
}

/// Checks if two line segments intersect. Returns true if they do, false otherwise.
fn do_lines_intersect(a: &LineSegment, b: &LineSegment) -> bool {
    let p = if let Some(p) = intersection_point(a, b) {
        p
    }
    else {
        return false;
    };

    // Check x bounds for both segments
    if p.x < a.start.x.min(a.end.x) || p.x > a.start.x.max(a.end.x) { return false; }
    if p.x < b.start.x.min(b.end.x) || p.x > b.start.x.max(b.end.x) { return false; }

    // Check y bounds for both segments
    if p.y < a.start.y.min(a.end.y) || p.y > a.start.y.max(a.end.y) { return false; }
    if p.y < b.start.y.min(b.end.y) || p.y > b.start.y.max(b.end.y) { return false; }

    true
}
