mod vecs;
mod game;

fn main() {
    let cb = ggez::ContextBuilder::new("micro_doom", ":P");
    let (mut ctx, event_loop) = cb.build().unwrap();
    let state = game::GameState::new(&mut ctx);
    ggez::event::run(ctx, event_loop, state);
}
