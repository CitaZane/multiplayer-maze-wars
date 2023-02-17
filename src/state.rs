use crate::main_menu::MainMenuStruct;
pub use crate::map::Map;
pub use crate::player::Player;
use crate::view::{remove_input_text_last_letter, View};
// pub use crate::view::View;
use ggez::event::{EventHandler, MouseButton};
use ggez::graphics::{self, Color};
use ggez::input::keyboard::{self, KeyCode};
use ggez::{Context, GameError, GameResult};

pub struct State {
    pub view: View,
}

impl State {
    pub fn new(ctx: &mut Context) -> GameResult<State> {
        Ok(State {
            view: View::MainMenu(MainMenuStruct::new(ctx)?),
        })
    }
}

impl EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if let View::Game(game_data) = &mut self.view {
            if ctx.keyboard.is_key_pressed(KeyCode::Up) {
                game_data.player.go_forward(&game_data.map.maze);
            }
            if ctx.keyboard.is_key_pressed(KeyCode::Down) {
                game_data.player.go_backward(&game_data.map.maze);
            }
            if ctx.keyboard.is_key_pressed(KeyCode::Left) {
                game_data.player.turn_left();
            }
            if ctx.keyboard.is_key_pressed(KeyCode::Right) {
                game_data.player.turn_right();
            }
        }

        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);
        // Draw code here...
        // self.map.draw(&mut canvas, ctx)?;
        self.view.draw(&mut canvas, ctx)?;
        // self.draw_scene(&mut canvas, ctx)?;
        // self.map.draw_player_position(&mut canvas, &self.player)?;
        // self.draw_fps_counter(&mut canvas, ctx)?;
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
