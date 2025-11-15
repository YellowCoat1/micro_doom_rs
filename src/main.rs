mod game;

fn main() {
    let cb = ggez::ContextBuilder::new("micro_doom", ":P");
    let (ctx, event_loop) = cb.build().unwrap();
    let state = game::GameState::new();
    ggez::event::run(ctx, event_loop, state);
}
