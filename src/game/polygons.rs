use ggez::{Context, graphics::{Canvas, Color}};

use super::vecs::Vec2;

pub struct Polygon {
    pub points: Vec<Vec2>,
}


impl Polygon {
    pub fn new(points: Vec<Vec2>) -> Self {
        Self { points }
    }
    pub fn draw_filled(&self, ctx: &mut Context, canvas: &mut Canvas, color: Color) {
        let poly = ggez::graphics::Mesh::new_polygon(
            ctx,
            ggez::graphics::DrawMode::fill(),
            &self.points.iter().map(|p| ggez::mint::Point2 { x: p.x, y: p.y }).collect::<Vec<_>>(),
            color,
        ).unwrap();
        canvas.draw(&poly, ggez::graphics::DrawParam::default())
    }
    pub fn draw_unfilled(&self, ctx: &mut Context, canvas: &mut Canvas, color: Color) {
        let poly = ggez::graphics::Mesh::new_polygon(
            ctx,
            ggez::graphics::DrawMode::stroke(1.5),
            &self.points.iter().map(|p| ggez::mint::Point2 { x: p.x, y: p.y }).collect::<Vec<_>>(),
            color,
        ).unwrap();
        canvas.draw(&poly, ggez::graphics::DrawParam::default())
    }
}

impl From<&[Vec2]> for Polygon {
    fn from(points: &[Vec2]) -> Self {
        Self::new(points.to_vec())
    }
}

impl From<Vec<Vec2>> for Polygon {
    fn from(points: Vec<Vec2>) -> Self {
        Self::new(points)
    }
}
