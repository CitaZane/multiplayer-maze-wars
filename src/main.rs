use ggez::conf::{Conf, WindowMode, WindowSetup};
use ggez::event;
use ggez::{ContextBuilder, GameResult};
use state::State;
pub mod client;
pub mod create_game;
pub mod drawer;
pub mod game;
pub mod join_game;
pub mod main_menu;
pub mod create_map;
pub mod map;
pub mod player;
pub mod server;
pub mod state;
pub mod view;
const SCREEN_WIDTH: f32 = 600.0;
const SCREEN_HEIGHT: f32 = 800.0;
const VIEWPORT_WIDTH: f32 = 370.0;
const VIEWPORT_HEIGHT: f32 = 410.0;

fn main() -> GameResult {
    let c = Conf::new();
    let window_mode = WindowMode::default().dimensions(SCREEN_WIDTH, SCREEN_HEIGHT);
    let window_setup = WindowSetup::default().title("Maze Wars").icon("/eye.png");
    let (mut ctx, event_loop) = ContextBuilder::new("maze_wars", "The Gang")
        .default_conf(c)
        .window_setup(window_setup)
        .window_mode(window_mode)
        .add_resource_path("resources")
        .build()?;
    let state = State::new(&mut ctx)?;
    event::run(ctx, event_loop, state);
}
