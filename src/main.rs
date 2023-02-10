use ggez::{ContextBuilder, GameResult};
use ggez::event;
use ggez::conf::{WindowMode, Conf, WindowSetup};

mod game;
pub use game::*;
pub mod map;


fn main()  -> GameResult {
    // Make a Context.
    let c = Conf::new();
    let window_mode = WindowMode::default().dimensions(606.0, 808.0);
    let window_setup = WindowSetup::default().title("Maze Wars");

    let (mut ctx, event_loop) = ContextBuilder::new("maze_wars", "The Gang")
        .default_conf(c)
        .window_setup(window_setup)
        .window_mode(window_mode)
        .build()?;

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let game = Game::new(&mut ctx);

    // Run!
    event::run(ctx, event_loop, game);
}
