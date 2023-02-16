use crate::{SCREEN_WIDTH, VIEWPORT_HEIGHT, VIEWPORT_WIDTH};

use ggez::{
    graphics::{self, Color, DrawParam, Mesh},
    Context, GameResult,
};

use crate::{drawer::Drawer, Map};
const X: f32 = (SCREEN_WIDTH - VIEWPORT_WIDTH) / 2.0;
const Y: f32 = 20.0;
pub struct GameStruct {
    pub drawer: Drawer,
    pub map: Map,
}

impl GameStruct {
    pub fn new(ctx: &mut Context) -> GameResult<GameStruct> {
        let drawer = Drawer::new(ctx)?;
        Ok(GameStruct {
            map: Map::new(),
            drawer,
        })
    }

    pub fn draw(&self, canvas: &mut graphics::Canvas, ctx: &mut Context) -> GameResult {
        self.map.draw(canvas, ctx)?;
        let frame = graphics::Rect::new(X, Y, VIEWPORT_WIDTH, VIEWPORT_HEIGHT);
        let mesh = Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::stroke(1.0),
            frame,
            Color::from_rgb(0, 0, 0),
        )?;
        canvas.draw(&mesh, DrawParam::default());

        Ok(())
    }
}
