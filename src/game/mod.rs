//Screen size is 800x600
mod vecs;
mod a3d_to_2d;
mod lines;
mod array;
mod polygons;
mod cam;
mod skybox;
mod fs;
use lines::LineSegment;
use ggez::{Context, GameResult, event, graphics, graphics::Color};
use ggez::input::keyboard::KeyCode;
use vecs::{Vec2, Vec3};
use rand::Rng;
use cam::Camera;
use polygons::Polygon;

use nalgebra_glm as glm;

use crate::game::a3d_to_2d::*;
use crate::game::lines::LineSegment3;

pub struct GameState {
    cam: Camera,
    walls:Vec<(LineSegment, Color)>
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
        let floor_plan: Vec<(Vec2, Vec2)> = vec![
            (Vec2::new(1.0, 10.0), Vec2::new(2.0, 6.0)),
            (Vec2::new(2.0, 6.0), Vec2::new(5.0, 5.0)),
            (Vec2::new(5.0, 5.0), Vec2::new(7.0, 7.0)),
            (Vec2::new(7.0, 7.0), Vec2::new(6.0, 10.0)),
            (Vec2::new(6.0, 10.0), Vec2::new(1.0, 10.0)),
        ];

        let camera3d: vecs::Vec3 = Default::default();
        let fov: f32 = 120.0_f32.to_radians();
        let cooler_floor_plan = floor_plan.into_iter()
            .map(|v| (v.into(), random_color()))
            .collect::<Vec<_>>();

        GameState {
            // Initialize game state here
            cam: Camera {
                pos: camera3d,
                fov,
                yaw: 0.0,
                near: 0.1,
            },
            walls: cooler_floor_plan,
        }
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        if ctx.keyboard.is_key_pressed(KeyCode::Up) {
            self.cam.pos.z += 0.01;
        }
        if ctx.keyboard.is_key_pressed(KeyCode::Down) {
            self.cam.pos.z -= 0.01;
        }
        if ctx.keyboard.is_key_pressed(KeyCode::Left) {
            self.cam.pos.x -= 0.01;
        }
        if ctx.keyboard.is_key_pressed(KeyCode::Right) {
            self.cam.pos.x += 0.01;
        }
        if ctx.keyboard.is_key_pressed(KeyCode::J) {
            self.cam.yaw -= 0.01;
        }
        if ctx.keyboard.is_key_pressed(KeyCode::K) {
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

    for (wall_segment, color) in game_state.walls.iter() {
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
        (poly.clone()).draw_filled(ctx, canvas, *color);
    }

    Ok(())
}
fn random_color() -> Color{
    let mut rng = rand::rng();
    let r: u8 = rng.random();
    let g: u8 = rng.random();
    let b: u8 = rng.random();
    Color::from_rgb(r, g, b)
}

