use micro_doom_rs::{GameState, GraphicsContext, KeysDown, Drawer};

struct GGEZGame {
    state: GameState,
}

impl GGEZGame {
    pub fn new() -> Self {
        Self {
            state: GameState::new(),
        }
    }
}

fn main() {
    let cb = ggez::ContextBuilder::new("micro_doom", ":P");
    let (ctx, event_loop) = cb.build().unwrap();
    let game = GGEZGame::new();
    ggez::event::run(ctx, event_loop, game);
}

use ggez::event;
use ggez::Context;
use ggez::GameResult;
use ggez::input::keyboard::KeyCode;
use ggez::graphics::{Canvas, Color};
use mint::Point2;


impl event::EventHandler for GGEZGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let keys_down = KeysDown {
            up: ctx.keyboard.is_key_pressed(KeyCode::Up),
            down: ctx.keyboard.is_key_pressed(KeyCode::Down),
            left: ctx.keyboard.is_key_pressed(KeyCode::Left),
            right: ctx.keyboard.is_key_pressed(KeyCode::Right),
            w: ctx.keyboard.is_key_pressed(KeyCode::W),
            s: ctx.keyboard.is_key_pressed(KeyCode::S),
        };

        let delta = ctx.time.delta().as_secs_f32();
        self.state.keys(delta, keys_down);

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let mut canvas = Canvas::from_frame(ctx, Color::WHITE);
        let (width, height) = ctx.gfx.size();
        let mut drawer = PolyDrawerGGEZ::new(ctx, &mut canvas);
        let mut graphics_context = GraphicsContext {
            width: width as u32,
            height: height as u32,
            drawer: &mut drawer,
        };
        self.state.draw_screen(&mut graphics_context);
        canvas.finish(ctx)
    }
}

pub struct PolyDrawerGGEZ<'a> {
    ctx: &'a mut ggez::Context,
    canvas: &'a mut ggez::graphics::Canvas,
}

impl<'a> PolyDrawerGGEZ<'a> {
    pub fn new(ctx: &'a mut ggez::Context, canvas: &'a mut ggez::graphics::Canvas) -> Self {
        Self { ctx, canvas }
    }
}

impl<'a> Drawer for PolyDrawerGGEZ<'a> {
    fn draw_polygon(&mut self, points: &[Point2<f32>], color: (u8, u8, u8, u8)) {
        use ggez::graphics::Color;
        let color = Color::from_rgba(color.0, color.1, color.2, color.3);
        let poly = ggez::graphics::Mesh::new_polygon(
            &self.ctx.gfx,
            ggez::graphics::DrawMode::fill(),
            &points
                .iter()
                .map(|p| ggez::mint::Point2 { x: p.x, y: p.y })
                .collect::<Vec<_>>(),
            color,
        )
        .unwrap();
        self.canvas
            .draw(&poly, ggez::graphics::DrawParam::default());
    }
}
