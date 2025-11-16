//Screen size is 800x600
mod vecs;
mod a3d_to_2d;
mod lines;
mod array;
mod polygons;
use lines::LineSegment;
use ggez::{Context, GameResult, event, graphics, graphics::Color};
use ggez::graphics::{DrawMode, Mesh};

use crate::game::vecs::Vec2;
pub struct GameState {
}



impl GameState {
    pub fn new(ctx: &mut Context) -> Self {
        let mut apoint3d: vecs::Vec3 = (10.0, 10.0, 10.0).into();
        let mut camera3d: vecs::Vec3 = (1.0, 1.0, 1.0).into();
        let camera_distance: f32 = 1.0;
        let apoint2d: vecs::Vec2 = a3d_to_2d::a3d_to_2d(apoint3d, camera3d, camera_distance).unwrap();
        println!("Point: {apoint2d:?}");

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

    let line_seg: LineSegment = (10.0, 10.0, 20.0, 20.0).into();

  /*   let line_seg = LineSegment {
        start: Vec2 {
            x: 0.0,
            y: 0.0,
        },
        end: Vec2 {
            x: 0.0,
            y: 600.0,
        }
    };
        let line_seg2 = LineSegment {
        start: Vec2 {
            x: 0.0,
            y: 0.0,
        },
        end: Vec2 {
            x: 800.0,
            y: 0.0,
        }
    };*/

    line_seg.draw(ctx, canvas, Color::BLACK);
    //line_seg2.draw(ctx, canvas, Color::BLACK);


    Ok(())
}
