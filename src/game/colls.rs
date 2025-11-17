use ggez::Context;

use super::GameState;
use super::lines::{LineSegment, do_lines_intersect};

//use super::vecs::Vec2;

//pub fn dot(a: Vec2, b: Vec2) -> f32 {
//    a.x * b.x + a.y * b.y
//}

// assumed to be forward
pub fn attempt_move(game_state: &mut GameState, ctx: &mut Context) {
    let delta = ctx.time.delta().as_secs_f32();
    let cam_pos = game_state.cam.pos;
    let forward = game_state.cam.forward_vector();
    let new_pos = game_state.cam.pos + forward * 8.0 * delta;

    let extended_ray = game_state.cam.pos + (forward * 50.0 * delta);

    let segment = LineSegment {
        start: cam_pos.into(),
        end: extended_ray.into(),
    };

    // for each wall segment, check if it intersects
    let intersects = game_state.bsp.order(cam_pos.into())
        .into_iter()
        .find(|wall_segment| {
            do_lines_intersect(&segment, wall_segment)
        });

    if intersects.is_some() {
        return;
    }

    game_state.cam.pos = new_pos;
}


