use ggez::{Context, graphics::{self, Mesh, Color, DrawParam, Text}, GameResult};

use crate::{
    create_game::CreateGameStruct, game::GameStruct, join_game::JoinGameStruct,
    main_menu::MainMenuStruct,
};
use crate::{VIEWPORT_WIDTH, VIEWPORT_HEIGHT, SCREEN_WIDTH};
const X: f32 = (SCREEN_WIDTH - VIEWPORT_WIDTH) / 2.0;
const Y: f32 = 20.0;
pub struct View2{
}
pub enum View {
    Game(GameStruct),
    MainMenu(MainMenuStruct),
    JoinGame(JoinGameStruct),
    CreateGame(CreateGameStruct),
}
impl View {
    pub fn draw(&self, canvas: &mut graphics::Canvas, ctx: &mut Context) -> GameResult {
        match self {
            View::Game(view) => view.draw(canvas, ctx)?,
            View::MainMenu(view) => view.draw(canvas, ctx)?,
            View::JoinGame(view) => view.draw(canvas, ctx)?,
            View::CreateGame(view) => view.draw(canvas, ctx)?,
        };
        Ok(())
    }
}
// removes last letter from text input and returns the new Text
pub fn remove_input_text_last_letter(mut text_input: String) -> Text {
    text_input.pop();
    Text::new(text_input)
}
impl View{
    pub fn new()-> Self{
        View2{}
    }
    pub fn draw(&self, canvas: &mut graphics::Canvas, ctx: &mut Context) -> GameResult{
        View2::draw_frame(canvas, ctx)?;   
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