//! something able to draw a polygon. Generally implementations are constructed at the start of the draw loop and dropped at the
//! end of it.
//!
//! This is so the entire thing can be picked up and dropped onto any rendering backend that can
//! draw to a window.

use super::vecs::Vec2;

pub trait Drawer {
    fn draw_polygon(&mut self, points: &[Vec2], color: (u8, u8, u8, u8));
    fn screen_width(&self) -> f32;
    fn screen_height(&self) -> f32;
}

