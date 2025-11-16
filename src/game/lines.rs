use ggez::{Context, graphics::{Canvas, Color, Mesh}, mint::Point2};
use std::ops::{Add, Sub, Mul};

use crate::game::vecs::Vec3;

use super::vecs::Vec2;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LineSegment {
    pub start: Vec2,
    pub end: Vec2,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LineSegment3 {
    pub start: Vec3,
    pub end: Vec3,
}

impl LineSegment3 {
    pub fn new(start: Vec3, end: Vec3) -> Self {
        Self { start, end }
    }

    pub fn length(&self) -> f32 {
        (self.end - self.start).length()
    }

    pub fn midpoint(&self) -> Vec3 {
        (self.start + self.end) * 0.5
    }
}


impl From<LineSegment3> for (Vec3, Vec3) {
    fn from(line: LineSegment3) -> Self {
        (line.start, line.end)
    }
}

impl From<(Vec3, Vec3)> for LineSegment3 {
    fn from(points: (Vec3, Vec3)) -> Self {
        Self {
            start: points.0,
            end: points.1,
        }
    }
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
pub fn do_lines_intersect(a: &LineSegment, b: &LineSegment) -> bool {
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

// Assuming that a interpreted as an inf line intersects with the line segment b,
// returns the 2 split segments of b from both sides of the intersection.
//
// Panics if the lines do not intersect (that means they're parallel. use do_lines_intersect or
// point_side_of_line to check)
/// Splits line segment `b` into two segments by intersecting it with the infinite line defined by `a`.
/// Panics if the lines are parallel or coincident.
pub fn split_line(a: &LineSegment, b: &LineSegment) -> Option<(LineSegment, LineSegment)> {
    let x1 = a.start.x;
    let y1 = a.start.y;
    let x2 = a.end.x;
    let y2 = a.end.y;

    let x3 = b.start.x;
    let y3 = b.start.y;
    let x4 = b.end.x;
    let y4 = b.end.y;

    let denom = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);
    if denom.abs() < f32::EPSILON {
        return None;
    }

    let px = ((x1 * y2 - y1 * x2) * (x3 - x4) - (x1 - x2) * (x3 * y4 - y3 * x4)) / denom;
    let py = ((x1 * y2 - y1 * x2) * (y3 - y4) - (y1 - y2) * (x3 * y4 - y3 * x4)) / denom;

    let intersection = Vec2 { x: px, y: py };


    let within_b = (intersection.x - x3) * (intersection.x - x4) <= 0.0
        && (intersection.y - y3) * (intersection.y - y4) <= 0.0;

    if !within_b {
        return None;
    }

    let seg1 = LineSegment {
        start: b.start,
        end: intersection,
    };

    let seg2 = LineSegment {
        start: intersection,
        end: b.end,
    };

    Some((seg1, seg2))
}

fn midpoint(seg: &LineSegment) -> Vec2 {
    Vec2 {
        x: (seg.start.x + seg.end.x) * 0.5,
        y: (seg.start.y + seg.end.y) * 0.5,
    }
}


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Order {
    Left,
    Right,
    On,
}

// determines which side of the line the point is on.
// Treats the line as infinite in both directions.
pub fn point_side_of_line(line: &LineSegment, point: &Vec2) -> Order {
    let val = (line.end.x - line.start.x) * (point.y - line.start.y) -
              (line.end.y - line.start.y) * (point.x - line.start.x);
    if val > 0.0 {
        Order::Left
    } else if val < 0.0 {
        Order::Right
    } else {
        Order::On
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_intersection_point() {
        let line1 = LineSegment::from((0.0, 0.0, 2.0, 2.0));
        let line2 = LineSegment::from((0.0, 2.0, 2.0, 0.0));
        let intersection = intersection_point(&line1, &line2).unwrap();
        assert_eq!(intersection, Vec2::new(1.0, 1.0));
    }
    #[test]
    fn test_no_intersection_point() {
        let line1 = LineSegment::from((0.0, 0.0, 1.0, 1.0));
        let line2 = LineSegment::from((0.0, 1.0, 1.0, 2.0));
        let intersection = intersection_point(&line1, &line2);
        assert!(intersection.is_none());
    }
    #[test]
    fn test_do_lines_intersect() {
        let line1 = LineSegment::from((0.0, 0.0, 2.0, 2.0));
        let line2 = LineSegment::from((0.0, 2.0, 2.0, 0.0));
        assert!(do_lines_intersect(&line1, &line2));
    }
    #[test]
    fn test_point_side_left() {
        let line = LineSegment::from((0.0, 0.0, 2.0, 0.0));
        let point = Vec2::new(1.0, 1.0);
        assert_eq!(point_side_of_line(&line, &point), Order::Left);
    }
    #[test]
    fn test_point_side_right() {
        let line = LineSegment::from((0.0, 0.0, 2.0, 0.0));
        let point = Vec2::new(1.0, -1.0);
        assert_eq!(point_side_of_line(&line, &point), Order::Right);
    }
    #[test]
    fn test_point_side_on() {
        let line = LineSegment::from((0.0, 0.0, 2.0, 0.0));
        let point = Vec2::new(1.0, 0.0);
        assert_eq!(point_side_of_line(&line, &point), Order::On);
    }
}
