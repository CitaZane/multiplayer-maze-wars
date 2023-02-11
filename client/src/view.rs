use ggez::{Context, graphics::{self, Mesh, Color, DrawParam}, GameResult};
use crate::{VIEWPORT_WIDTH, VIEWPORT_HEIGHT, SCREEN_WIDTH};
const X: f32 = (SCREEN_WIDTH - VIEWPORT_WIDTH) / 2.0;
const Y: f32 = 20.0;
pub struct View{
}
impl View{
    pub fn new()-> Self{
        View{}
    }
    pub fn draw(&self, canvas: &mut graphics::Canvas, ctx: &mut Context) -> GameResult{
        View::draw_frame(canvas, ctx)?;   
        Ok(())
    }
    fn draw_frame(canvas: &mut graphics::Canvas, ctx: &mut Context) -> GameResult{
        let frame = graphics::Rect::new(X, Y, VIEWPORT_WIDTH,VIEWPORT_HEIGHT);
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