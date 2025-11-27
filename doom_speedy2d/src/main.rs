use mint::Point2;
use speedy2d::dimen::Vec2;
use speedy2d::window::{VirtualKeyCode, WindowHelper};
use speedy2d::{Window, window::WindowHandler};
use speedy2d::color::Color;
use shared::{Drawer, GameState, GraphicsContext, KeysDown};
use std::time::Instant;

fn main() {
    let window = Window::new_centered("My Speedy2D Window", (800, 600)).unwrap();
    window.run_loop(WindowState::default());
}

struct WindowState {
    game_state: GameState,
    keys_down: std::collections::HashSet<VirtualKeyCode>,
    last: Instant,
}

impl Default for WindowState {
    fn default() -> Self {
        Self {
            game_state: GameState::new(),
            keys_down: std::collections::HashSet::new(),
            last: Instant::now(),
        }
    }
}

impl WindowHandler for WindowState {

    fn on_key_down(
        &mut self,
        _helper: &mut WindowHelper<()>,
        key: Option<VirtualKeyCode>,
        _scancode: speedy2d::window::KeyScancode,
    ) {
        if let Some(k) = key {
            self.keys_down.insert(k);
        }
    }

    fn on_key_up(
        &mut self,
        _helper: &mut WindowHelper<()>,
        key: Option<VirtualKeyCode>,
        _scancode: speedy2d::window::KeyScancode,
    ) {
        if let Some(k) = key {
            self.keys_down.remove(&k);
        }
    }

    fn on_draw(
            &mut self,
            helper: &mut speedy2d::window::WindowHelper<()>,
            graphics: &mut speedy2d::Graphics2D
        ) {
        let now = Instant::now();
        let dt = now.duration_since(self.last).as_secs_f32();
        self.last = now;
        graphics.clear_screen(Color::from_rgb(1.0, 1.0, 1.0));

        let mut drawer = SpeedyDrawer { graphics };
        let size = helper.get_size_pixels();
        let mut gctx = GraphicsContext {
            drawer: &mut drawer,
            width: size.x,
            height: size.y,
        };

        self.game_state.draw_screen(&mut gctx);



        let keys_down = KeysDown {
            up: self.keys_down.contains(&VirtualKeyCode::Up),
            down: self.keys_down.contains(&VirtualKeyCode::Down),
            left: self.keys_down.contains(&VirtualKeyCode::Left),
            right: self.keys_down.contains(&VirtualKeyCode::Right),
            w: self.keys_down.contains(&VirtualKeyCode::W),
            s: self.keys_down.contains(&VirtualKeyCode::S),
        };

        self.game_state.keys(dt, keys_down);
        // Request that we draw another frame once this one has finished
        helper.request_redraw();
    }

}

struct SpeedyDrawer<'a> {
    graphics: &'a mut speedy2d::Graphics2D,
}

impl Drawer for SpeedyDrawer<'_> {
    fn draw_polygon(&mut self, points: &[mint::Point2<f32>], color: (u8, u8, u8, u8)) {
        // sort the points in clockwise order
        // calculate centroid
        let centroid = {
            let (sum_x, sum_y) = points.iter().fold((0.0, 0.0), |(sx, sy), p| (sx + p.x, sy + p.y));
            let count = points.len() as f32;
            Point2 { x: sum_x / count, y: sum_y / count }
        };
        let mut points = points.to_vec();
        points.sort_by(|a, b| {
            let angle_a = (a.y - centroid.y).atan2(a.x - centroid.x);
            let angle_b = (b.y - centroid.y).atan2(b.x - centroid.x);
            angle_a.partial_cmp(&angle_b).unwrap()
        });
        let polygon = speedy2d::shape::Polygon::new(
            points.iter().map(|p| Vec2::new(p.x, p.y)).collect::<Vec<_>>().as_slice(),
        );

        self.graphics.draw_polygon(
            &polygon,
            (0.0, 0.0),
            Color::from_int_rgba(color.0, color.1, color.2, color.3),
        );
    }
}
