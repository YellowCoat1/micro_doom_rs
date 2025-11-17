//! something able to draw a polygon. Generally implementations are constructed at the start of the draw loop and dropped at the
//! end of it.
//!
//! This is so the entire thing can be picked up and dropped onto any rendering backend that can
//! draw to a window.

use ggez::{Context, graphics::{Canvas, Color}};
use super::vecs::Vec2;

pub trait Drawer {
    fn draw_polygon(&mut self, points: &[Vec2], color: (u8, u8, u8, u8));
}


pub struct PolyDrawerGGEZ<'a> {
    ctx: &'a mut Context,
    canvas: &'a mut Canvas,
}

impl<'a> PolyDrawerGGEZ<'a> {
    pub fn new(ctx: &'a mut ggez::Context, canvas: &'a mut ggez::graphics::Canvas) -> Self {
        Self { ctx, canvas }
    }
}

impl<'a> Drawer for PolyDrawerGGEZ<'a> {
    fn draw_polygon(&mut self, points: &[Vec2], color: (u8, u8, u8, u8)) {
        let color = Color::from_rgba(color.0, color.1, color.2, color.3);
        let poly = ggez::graphics::Mesh::new_polygon(
            &self.ctx.gfx,
            ggez::graphics::DrawMode::fill(),
            &points.iter().map(|p| ggez::mint::Point2 { x: p.x, y: p.y }).collect::<Vec<_>>(),
            color,
        ).unwrap();
        self.canvas.draw(&poly, ggez::graphics::DrawParam::default());
    }
}
