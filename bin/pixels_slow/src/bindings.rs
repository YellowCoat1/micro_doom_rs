use shared::{Drawer, KeysDown};
use winit::keyboard::KeyCode;
use winit_input_helper::WinitInputHelper;

use crate::lines::PixelDrawer;

impl<'a> Drawer for PixelDrawer<'a> {
    fn draw_polygon(&mut self, points: &[mint::Point2<f32>], color: (u8, u8, u8, u8)) {
        let p = |n: usize| (points[n].x as i32, points[n].y as i32);
        let color = super::Color(color.0, color.1, color.2);
        let len = points.len();
        if len > 4 {
            let points = points
                .iter()
                .map(|pt| (pt.x as i32, pt.y as i32))
                .collect::<Vec<(i32, i32)>>();
            self.draw_poly(&points, color);
        } else if len == 4 {
            self.draw_quad(p(0), p(1), p(2), p(3), color);
        } else if points.len() == 3 {
            self.triangle(p(0), p(1), p(2), color);
        } else if points.len() == 2 {
            let p1 = p(0);
            let p2 = p(1);
            self.draw_line(p1.0, p1.1, p2.0, p2.1, color);
        } else {
            println!("Unsupported polygon with {} points", points.len());
        }
    }
}


pub fn keys(input: &WinitInputHelper) -> KeysDown {
    KeysDown {
        up: input.key_held(KeyCode::ArrowUp),
        down: input.key_held(KeyCode::ArrowDown),
        left: input.key_held(KeyCode::ArrowLeft),
        right: input.key_held(KeyCode::ArrowRight),
        w: input.key_held(KeyCode::KeyW),
        s: input.key_held(KeyCode::KeyS),
    }
}
