use ggez::conf::{Conf, WindowMode, WindowSetup};
use ggez::event;
use ggez::{ContextBuilder, GameResult};
mod game;
pub use game::*;
pub mod drawer;
pub mod map;
pub mod server;
pub mod views;
const SCREEN_WIDTH: f32 = 600.0;
const SCREEN_HEIGHT: f32 = 800.0;

fn main() -> GameResult {
    // env::set_var("RUST_BACKTRACE", "1");
    let c = Conf::new();
    let window_mode = WindowMode::default().dimensions(SCREEN_WIDTH, SCREEN_HEIGHT);
    let window_setup = WindowSetup::default().title("Maze Wars").icon("/eye.png");
    let (mut ctx, event_loop) = ContextBuilder::new("maze_wars", "The Gang")
        .default_conf(c)
        .window_setup(window_setup)
        .window_mode(window_mode)
        .add_resource_path("resources")
        .build()?;

    let game = Game::new(&mut ctx)?;
    event::run(ctx, event_loop, game);
}
