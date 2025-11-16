
mod vecs;
mod a3d_to_2d;
mod lines;
use lines::LineSegment;
use ggez::{Context, GameResult, event, graphics, graphics::Color};
use ggez::graphics::{DrawMode, Mesh};

use crate::game::vecs::Vec2;
pub struct GameState {
}



impl GameState {
    pub fn new(ctx: &mut Context) -> Self {
        GameState {
            // Initialize game state here



        }
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        // Update game logic here
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
    let line_seg = LineSegment {
        start: Vec2 {
            x: 100.0,
            y: 100.0,
        },
        end: Vec2 {
            x: 200.0,
            y: 200.0,
        }
    };

    line_seg.draw(ctx, canvas, Color::BLACK);


    Ok(())
}
