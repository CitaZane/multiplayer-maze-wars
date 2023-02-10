use ggez::{Context,GameResult};
use ggez::graphics::{self, Color};
use ggez::event::EventHandler;
pub use crate::map::Map;
pub use crate::view::View;

pub const VIEWPORT_WIDTH: f32 = 370.0;
pub const VIEWPORT_HEIGHT: f32 = 410.0;

pub struct Game {
    // Your state here...
    // viewport
    map: Map,
    view:View,
}
// 17 x 33
impl Game {
    pub fn new(_ctx: &mut Context) -> Self {
        // Load/create resources such as images here.
        Self {
            map: Map::new(),
            view: View::new(),
            // ...
        }
    }
}

impl EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // Update code here...
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);
        // Draw code here...
        self.map.draw(&mut canvas, ctx)?;
        self.view.draw(&mut canvas, ctx)?;
        canvas.finish(ctx)
    }
}