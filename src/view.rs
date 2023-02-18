use ggez::{
    graphics::{self, Text},
    Context, GameResult,
};

use crate::{
    create_game::CreateGameStruct, game::GameStruct, join_game::JoinGameStruct,
    main_menu::MainMenuStruct,
};
pub enum View {
    Game(GameStruct),
    MainMenu(MainMenuStruct),
    JoinGame(JoinGameStruct),
    CreateGame(CreateGameStruct),
}
impl View {
    pub fn draw(&mut self, canvas: &mut graphics::Canvas, ctx: &mut Context) -> GameResult {
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
