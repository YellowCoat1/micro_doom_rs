use crate::game::drawing::Drawer;
use ggez::graphics::{self, Color};
use ggez::{Context, GameResult};
use nalgebra_glm as glm;
use once_cell::sync::Lazy;

use super::GameState;
use super::a3d_to_2d;
use super::lines::LineSegment3;
use super::skybox;
use super::vecs::{Vec2, Vec3};

use rand::Rng;

static RAND_32: Lazy<u32> = Lazy::new(|| rand::rng().random());

pub fn draw_screen(
    game_state: &mut GameState,
    ctx: &mut Context,
    canvas: &mut graphics::Canvas,
) -> GameResult<()> {

    // the projection matrix
    // This is the calculation of the matrix that converts 3D points to 2D screen points.
    // We use a right-handed coordinate system with zero to one depth range.
    let (width, height) = ctx.gfx.size();
    let proj = glm::perspective_rh_zo(
        width / height,
        game_state.cam.fov,
        game_state.cam.near,
        1000.0,
    );




    let mut drawer = super::drawing::PolyDrawerGGEZ::new(ctx, canvas);
    let cam_pos_2d = Vec2 {
        x: game_state.cam.pos.x,
        y: game_state.cam.pos.z,
    };
    skybox::draw_skybox(&game_state.cam, &mut drawer, proj);

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
        

        let conv_wall_point_set: Vec<glm::Vec3> = wall_point_set
            .iter()
            .map(|v| glm::vec3(v.x, v.y, v.z))
            .collect();

        let screen_coord = a3d_to_2d::clip_and_project_polygon(
            &conv_wall_point_set,
            &game_state.cam,
            proj,
            width,
            height,
        );
        if screen_coord.len() < 3 {
            continue;
        }
        // draw poly
        drawer.draw_polygon(&screen_coord, color.to_rgba());
    }

    Ok(())
}

fn random_color(v: (Vec2, Vec2)) -> Color {
    let mut rng = *RAND_32
        + (v.0.x as u32) * 100
        + (v.0.y as u32) * 1000
        + (v.1.x as u32) * 5000
        + (v.1.y as u32) * 10000;
    let r = ((rng & 0xFF0000) >> 16) as u8;
    rng = rng.wrapping_mul(1103515245).wrapping_add(12345);
    let g = ((rng & 0x00FF00) >> 8) as u8;
    rng = rng.wrapping_mul(1103515245).wrapping_add(12345);
    let b = (rng & 0x0000FF) as u8;
    Color::from_rgb(r, g, b)
}

fn wall_floor_to_3d(wall_left: &Vec2, wall_right: &Vec2) -> (LineSegment3, LineSegment3) {
    let base = -0.75;
    let offset_up = 1.75;
    let line_seg = LineSegment3 {
        start: Vec3 {
            x: wall_left.x,
            y: base,
            z: wall_left.y,
        },
        end: Vec3 {
            x: wall_right.x,
            y: base,
            z: wall_right.y,
        },
    };
    let line_seg_top = LineSegment3 {
        start: Vec3 {
            x: wall_left.x,
            y: base + offset_up,
            z: wall_left.y,
        },
        end: Vec3 {
            x: wall_right.x,
            y: base + offset_up,
            z: wall_right.y,
        },
    };
    (line_seg, line_seg_top)
}
