use ggez::{Context, GameResult, graphics::{self, Rect}};

use crate::game::GameState;

pub fn draw_skybox(_game_state: &mut GameState, ctx: &mut Context, canvas: &mut graphics::Canvas, width: f32, height: f32) -> GameResult<()> {
    let rectangle = Rect::new(0.0, 0.0, width, height/2.0);
    let rect_mesh = graphics::Mesh::new_rectangle(&ctx.gfx, graphics::DrawMode::fill(), rectangle, graphics::Color::BLUE)?;
    canvas.draw(&rect_mesh, graphics::DrawParam::default());
    Ok(())
}
