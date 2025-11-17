use ggez::input::keyboard::KeyCode;
use ggez::{Context, GameResult, event, graphics, graphics::Color};
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
use cam::Camera;
use colls::attempt_move;
use lines::LineSegment;
use vecs::Vec3;

use bsp::BSPNode;

pub struct GameState {
    cam: Camera,
    bsp: BSPNode,
}
impl GameState {
    pub fn new(_ctx: &mut Context) -> Self {
        //let mut apoint3d: vecs::Vec3 = (10.0, 10.0, 10.0).into();
        //let mut another_point3d: vecs::Vec3 = (10.0, 20.0, 10.0).into();
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
                near: 0.1,
            },
            bsp,
        }
    }
}
impl event::EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let forward = self.cam.forward_vector();
        dbg!("Camera pos: {:?}", self.cam.pos);
        let delta = ctx.time.delta().as_secs_f32();
        if ctx.keyboard.is_key_pressed(KeyCode::Up) {
            attempt_move(self, ctx);
        }
        if ctx.keyboard.is_key_pressed(KeyCode::Down) {
            self.cam.pos = self.cam.pos - forward * 3.0 * delta;
        }
        if ctx.keyboard.is_key_pressed(KeyCode::Left) {
            self.cam.yaw -= 0.01;
        }
        if ctx.keyboard.is_key_pressed(KeyCode::Right) {
            self.cam.yaw += 0.01;
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);

        draw_screen::draw_screen(self, ctx, &mut canvas)?;

        canvas.finish(ctx)
    }
}
