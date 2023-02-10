use ggez::{Context,GameResult};
use ggez::graphics::{self, Color};
use ggez::event::EventHandler;

pub struct Game {
    // Your state here...
}

impl Game {
    pub fn new(_ctx: &mut Context) -> Self {
        // Load/create resources such as images here.
        Self {
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
        canvas.finish(ctx)
    }
}