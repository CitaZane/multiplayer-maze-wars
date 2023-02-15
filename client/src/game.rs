pub use crate::map::Map;
pub use crate::view::View;
use crate::view::{remove_input_text_last_letter, ViewType};
use ggez::event::{EventHandler, MouseButton};
use ggez::graphics::{self, Color, Drawable};
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
            view: View::new(ctx, ViewType::MainMenu)?,
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
        _ctx: &mut Context,
        button: MouseButton,
        x: f32,
        y: f32,
    ) -> Result<(), GameError> {
        if let MouseButton::Left = button {
            // if current view will change, it will hold the new view value
            let mut new_view = None;
            // make inputs inactive in case if user clicked on anything else than a input box
            self.view.name_input_active = false;
            self.view.ip_input_active = false;

            // based on current view, look if user clicked on its element rects
            // element rects are input and buttons rects
            match &self.view.current_view {
                ViewType::MainMenu => {
                    for (name, elem_rect) in &self.view.element_rects {
                        if x > elem_rect.x
                            && x < elem_rect.x + elem_rect.w
                            && y > elem_rect.y
                            && y < elem_rect.y + elem_rect.h
                        {
                            if name == "CREATE_GAME" {
                                new_view = Some(ViewType::CreateGame);
                            } else if name == "JOIN_GAME" {
                                new_view = Some(ViewType::JoinGame);
                            }
                        }
                    }
                }
                ViewType::JoinGame => {
                    for (name, elem_rect) in &self.view.element_rects {
                        if x > elem_rect.x
                            && x < elem_rect.x + elem_rect.w
                            && y > elem_rect.y
                            && y < elem_rect.y + elem_rect.h
                        {
                            if name == "IP_INPUT" {
                                self.view.ip_input_active = true;
                                if self.view.name_input_active {
                                    self.view.name_input_active = false;
                                }
                            } else if name == "NAME_INPUT" {
                                self.view.name_input_active = true;
                                if self.view.ip_input_active {
                                    self.view.ip_input_active = false;
                                }
                            } else if name == "JOIN_GAME" {
                                new_view = Some(ViewType::Game(Map::new()));
                            } else if name == "BACK_ARROW_IMG" {
                                new_view = Some(ViewType::MainMenu)
                            }
                        }
                    }
                }
                ViewType::CreateGame => {
                    for (name, elem_rect) in &self.view.element_rects {
                        if x > elem_rect.x
                            && x < elem_rect.x + elem_rect.w
                            && y > elem_rect.y
                            && y < elem_rect.y + elem_rect.h
                        {
                            if name == "NAME_INPUT" {
                                self.view.name_input_active = true;
                            } else if name == "CREATE_GAME" {
                                new_view = Some(ViewType::Game(Map::new()));
                            } else if name == "BACK_ARROW_IMG" {
                                new_view = Some(ViewType::MainMenu)
                            }
                        }
                    }
                }
                ViewType::Game(_) => {}
            };
            if let Some(view) = new_view {
                self.view = View::new(_ctx, view)?;
            }
        }

        Ok(())
    }

    fn text_input_event(&mut self, _ctx: &mut Context, character: char) -> Result<(), GameError> {
        if self.view.ip_input_active
            && character.is_alphanumeric()
            && self.view.ip_address.contents().len() <= 20
            || character == '.'
            || character == ':'
        {
            self.view.ip_address.add(character);
        }
        if self.view.name_input_active
            && character.is_alphanumeric()
            && self.view.name.contents().len() <= 10
        {
            self.view.name.add(character);
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
                if self.view.ip_input_active {
                    self.view.ip_address =
                        remove_input_text_last_letter(self.view.ip_address.contents());
                }
                if self.view.name_input_active {
                    self.view.name = remove_input_text_last_letter(self.view.name.contents());
                }
            }
        }
        Ok(())
    }
}
