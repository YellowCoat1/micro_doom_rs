use nalgebra_glm as glm;
use ggez::{
    Context, GameResult,
    graphics::{self, Rect},
};

use crate::game::GameState;

pub fn draw_skybox(
    game_state: &mut GameState,
    ctx: &mut Context,
    canvas: &mut graphics::Canvas,
    width: f32,
    height: f32,
    proj: glm::Mat4,
) -> GameResult<()> {

    let middlepoint = game_state.cam.pos + game_state.cam.forward_vector_zero_pitch();
    let screen_middlepoint = super::a3d_to_2d::project_point(middlepoint.into(), &game_state.cam, proj, width, height)
        .expect("screen middlepoint was off screen, somehow.");
    let y_val = screen_middlepoint.y;


    let rectangle = Rect::new(0.0, 0.0, width, y_val);
    let rect_mesh = graphics::Mesh::new_rectangle(
        &ctx.gfx,
        graphics::DrawMode::fill(),
        rectangle,
        graphics::Color::BLUE,
    )?;
    canvas.draw(&rect_mesh, graphics::DrawParam::default());
    Ok(())
}
