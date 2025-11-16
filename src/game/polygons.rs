use ggez::{Context, graphics::{Canvas, Color}};

use super::vecs::Vec2;

pub struct Polygon {
    pub points: Vec<Vec2>,
}

impl Polygon {
    pub fn new(points: Vec<Vec2>) -> Self {
        Self { points }
    }
    pub fn draw(&self, ctx: &mut Context, canvas: &mut Canvas, color: Color) {
        let poly = ggez::graphics::Mesh::new_polygon(
            ctx,
            ggez::graphics::DrawMode::fill(),
            &self.points.iter().map(|p| ggez::mint::Point2 { x: p.x, y: p.y }).collect::<Vec<_>>(),
            color,
        ).unwrap();
        canvas.draw(&poly, ggez::graphics::DrawParam::default())
    }
}
