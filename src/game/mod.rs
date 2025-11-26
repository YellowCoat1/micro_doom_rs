mod a3d_to_2d;
mod bsp;
mod cam;
mod colls;
mod draw_screen;
mod drawing;
mod fs;
mod lines;
mod skybox;
mod vecs;
mod contexts;

use cam::Camera;
use colls::attempt_move;
use lines::LineSegment;
use vecs::Vec3;
use bsp::BSPNode;

pub use drawing::Drawer;
pub use contexts::{GraphicsContext, KeysDown};

pub struct GameState {
    cam: Camera,
    bsp: BSPNode,
}
impl GameState {
    pub fn new() -> Self {

        let (floor_plan, cam_pos) = fs::segs_from_file();
        let camera3d: vecs::Vec3 = Vec3 {
            x: cam_pos.x,
            y: 0.0,
            z: cam_pos.y,
        };
        let fov: f32 = 80.0_f32.to_radians();

        let floor_plan_segs = floor_plan
            .iter()
            .map(|(s, e)| LineSegment { start: *s, end: *e })
            .collect::<Vec<LineSegment>>();

        let bsp = BSPNode::new(floor_plan_segs);

        GameState {
            // Initialize game state here
            cam: Camera {
                pos: camera3d,
                fov,
                yaw: 0.0,
                pitch: 0.0,
                near: 0.1,
            },
            bsp,
        }
    }
    pub fn draw_screen<T: Drawer>(&mut self, graphics_context: &mut GraphicsContext<'_, T>) {
        draw_screen::draw_screen(self, graphics_context);
    }
    pub fn keys(&mut self, delta: f32, keys_down: contexts::KeysDown) {
        let forward = self.cam.forward_vector();

        let flattened = {
            let mut flat = forward.clone();
            flat.y = 0.0;
            flat
        };
        
        if keys_down.up {
            attempt_move(self, delta, &flattened);
        }
        if keys_down.down {
            self.cam.pos = self.cam.pos - flattened * 3.0 * delta;
        }
        if keys_down.left {
            self.cam.yaw -= 1.0 * delta;
        }
        if keys_down.right {
            self.cam.yaw += 1.0 * delta;
        }
        if keys_down.w {
            self.cam.pitch -= 0.01;
        }
        if keys_down.s {
            self.cam.pitch += 0.01;
        }

    }
}

