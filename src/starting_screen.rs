pub use crate::map::Map;
use crate::main_menu::MainMenuStruct;
// use crate::remove_input_text_last_letter;
use crate::view::{View, remove_input_text_last_letter};
use ggez::event::{EventHandler, MouseButton};
use ggez::graphics::{self, Color};
use ggez::input::keyboard;
use ggez::{Context, GameError, GameResult};
pub const VIEWPORT_WIDTH: f32 = 370.0;
pub const VIEWPORT_HEIGHT: f32 = 410.0;
pub struct Game {
    view: View,
}
// 17 x 33
impl Game {
    pub fn new(ctx: &mut Context) -> GameResult<Game> {
        Ok(Self {
            view: View::MainMenu(MainMenuStruct::new(ctx)?),
        })
    }
}
impl EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);
        self.view.draw(&mut canvas, ctx)?;
        canvas.finish(ctx)
    }

    fn mouse_button_down_event(
        &mut self,
        ctx: &mut Context,
        button: MouseButton,
        x: f32,
        y: f32,
    ) -> Result<(), GameError> {
        if let MouseButton::Left = button {
            let mut new_view = None;
            match &mut self.view {
                View::MainMenu(view_data) => {
                    new_view = view_data.check_mouse_click(x, y, ctx);
                }
                View::JoinGame(view_data) => {
                    view_data.ip_input_active = false;
                    view_data.name_input_active = false;
                    new_view = view_data.check_mouse_click(x, y, ctx);
                }
                View::CreateGame(view_data) => {
                    view_data.name_input_active = false;
                    new_view = view_data.check_mouse_click(x, y, ctx);
                }
                View::Game(_) => {}
            };
            if let Some(view) = new_view {
                self.view = view;
            }
        }

        Ok(())
    }

    fn text_input_event(&mut self, _ctx: &mut Context, character: char) -> Result<(), GameError> {
        match &mut self.view {
            View::Game(_) => {}
            View::MainMenu(_) => {}
            View::JoinGame(view_data) => {
                if view_data.ip_input_active
                    && character.is_alphanumeric()
                    && view_data.ip_address.contents().len() <= 20
                    || character == '.'
                    || character == ':'
                {
                    view_data.ip_address.add(character);
                }
                if view_data.name_input_active
                    && character.is_alphanumeric()
                    && view_data.name.contents().len() <= 10
                {
                    view_data.name.add(character);
                }
            }
            View::CreateGame(view_data) => {
                if view_data.name_input_active
                    && character.is_alphanumeric()
                    && view_data.name.contents().len() <= 10
                {
                    view_data.name.add(character);
                }
            }
        }

        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        input: ggez::input::keyboard::KeyInput,
        _repeated: bool,
    ) -> Result<(), GameError> {
        if let Some(keycode) = input.keycode {
            if let keyboard::KeyCode::Back = keycode {
                match &mut self.view {
                    View::Game(_) => {}
                    View::MainMenu(_) => {}
                    View::JoinGame(view_data) => {
                        if view_data.ip_input_active {
                            view_data.ip_address =
                                remove_input_text_last_letter(view_data.ip_address.contents());
                        }
                        if view_data.name_input_active {
                            view_data.name =
                                remove_input_text_last_letter(view_data.name.contents());
                        }
                    }
                    View::CreateGame(view_data) => {
                        if view_data.name_input_active {
                            view_data.name =
                                remove_input_text_last_letter(view_data.name.contents());
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
