//Screen size is 800x600
mod vecs;
mod a3d_to_2d;
mod lines;
mod array;
mod polygons;
mod cam;
mod skybox;
mod fs;
mod bsp;
use lines::LineSegment;
use ggez::{Context, GameResult, event, graphics, graphics::Color};
use ggez::input::keyboard::KeyCode;
use vecs::{Vec2, Vec3};
use rand::Rng;
use cam::Camera;
use polygons::Polygon;
use once_cell::sync::Lazy;

use nalgebra_glm as glm;

static RAND_32: Lazy<u32> = Lazy::new(|| {
    rand::rng().random()
});

use crate::game::lines::LineSegment3;

use bsp::BSPNode;


pub struct GameState {
    cam: Camera,
    bsp: BSPNode,
}

fn wall_floor_to_3d(wall_left: &Vec2, wall_right: &Vec2) -> (LineSegment3, LineSegment3) {
    let base = -0.75;
    let offset_up = 1.75;
    let line_seg = LineSegment3 {
        start: Vec3 {
            x: wall_left.x,
            y: base,
            z: wall_left.y
        },
        end: Vec3 {
            x: wall_right.x,
            y: base,
            z: wall_right.y
        }
    };
    let line_seg_top = LineSegment3 {
        start: Vec3 {
            x: wall_left.x,
            y: base+offset_up,
            z: wall_left.y
        },
        end: Vec3 {
            x: wall_right.x,
            y: base+offset_up,
            z: wall_right.y
        }
    };
    (line_seg, line_seg_top)
}

impl GameState {
    pub fn new(ctx: &mut Context) -> Self {
        //let mut apoint3d: vecs::Vec3 = (10.0, 10.0, 10.0).into();
        //let mut another_point3d: vecs::Vec3 = (10.0, 20.0, 10.0).into();
        let (floor_plan, cam_pos) = fs::segs_from_file();
        let camera3d: vecs::Vec3 = Vec3 {
            x: cam_pos.x,
            y: 0.0,
            z: cam_pos.y,
        };
        let fov: f32 = 80.0_f32.to_radians();
        
        let floor_plan_segs = floor_plan.iter()
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
            self.cam.pos = self.cam.pos + forward * 3.0 * delta;
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

        draw_screen(self, ctx, &mut canvas)?;

        canvas.finish(ctx)
    }
}

fn draw_screen(game_state: &mut GameState, ctx: &mut Context, canvas: &mut graphics::Canvas) -> GameResult<()> {
    // Drawing logic here
    let (width, height) = ctx.gfx.size();

    skybox::draw_skybox(game_state, ctx, canvas, width as f32, height as f32)?;

    let cam_pos_2d = Vec2 {
        x: game_state.cam.pos.x,
        y: game_state.cam.pos.z,
    };

    let out_vec = game_state.bsp.order(cam_pos_2d);

    for wall_segment in out_vec.iter() {
        let color = random_color((wall_segment.start, wall_segment.end));
        //let rotated_wall_seg = cam::rotate_seg(*wall_segment, &game_state.cam);
        let wall_3d_segs = wall_floor_to_3d(&wall_segment.start, &wall_segment.end);

        let wall_point_set: Vec<Vec3> = vec![
            wall_3d_segs.0.start,
            wall_3d_segs.0.end,
            wall_3d_segs.1.end,
            wall_3d_segs.1.start,
        ];

        // the projection matrix
        // idk what this is tbh :(
        let proj = glm::perspective_rh_zo(
            width as f32 / height as f32,
            game_state.cam.fov,
            game_state.cam.near,
            1000.0,
        );

        let conv_wall_point_set : Vec<glm::Vec3> = wall_point_set.iter()
            .map(|v| glm::vec3(v.x, v.y, v.z))
            .collect();

        let screen_coord = a3d_to_2d::clip_and_project_polygon(&conv_wall_point_set, &game_state.cam, proj, width, height);
        if screen_coord.len() < 3 {
            continue;
        }
        // draw poly
        let poly = Polygon::new(screen_coord);
        (poly.clone()).draw_filled(ctx, canvas, color);
    }

    Ok(())
}
fn random_color(v: (Vec2, Vec2)) -> Color{
    let mut rng = *RAND_32 + (v.0.x as u32)*100 + (v.0.y as u32)*1000 +
        (v.1.x as u32)*5000 + (v.1.y as u32)*10000;
    let r = ((rng & 0xFF0000) >> 16) as u8;
    rng = rng.wrapping_mul(1103515245).wrapping_add(12345);
    let g = ((rng & 0x00FF00) >> 8) as u8;
    rng = rng.wrapping_mul(1103515245).wrapping_add(12345);
    let b = (rng & 0x0000FF) as u8;
    Color::from_rgb(r, g, b)
}

