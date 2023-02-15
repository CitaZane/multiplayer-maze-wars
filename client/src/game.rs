pub use crate::map::Map;
pub use crate::view::View;
use crate::view::ViewType;
use ggez::event::{EventHandler, MouseButton};
use ggez::graphics::{self, Color, Text};
use ggez::input::keyboard;
use ggez::{Context, GameError, GameResult};
pub const VIEWPORT_WIDTH: f32 = 370.0;
pub const VIEWPORT_HEIGHT: f32 = 410.0;
pub struct Game {
    // Your state here...
    // viewport
    map: Map,
    view: View,
}
// 17 x 33
impl Game {
    pub fn new(ctx: &mut Context) -> GameResult<Game> {
        Ok(Self {
            map: Map::new(),
            view: View::new(ctx, ViewType::MainMenu)?,
        })
    }
}
impl EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // Update code here...
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
            let mut new_view = None;
            self.view.name_input_active = false;
            self.view.ip_input_active = false;
            match &self.view.current_view {
                ViewType::Game(_) => {}
                ViewType::MainMenu => {
                    for (name, elem_rect) in &self.view.element_rects {
                        if x > elem_rect.x
                            && x < elem_rect.x + elem_rect.w
                            && y > elem_rect.y
                            && y < elem_rect.y + elem_rect.h
                        {
                            if name == "CREATE_GAME" {
                                new_view = Some(ViewType::CreateGame);

                                println!("Create game");
                            } else if name == "JOIN_GAME" {
                                new_view = Some(ViewType::JoinGame);
                                println!("Join game");
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
                                println!("IP input");
                                self.view.ip_input_active = true;
                                if self.view.name_input_active {
                                    self.view.name_input_active = false;
                                }
                            } else if name == "NAME_INPUT" {
                                self.view.name_input_active = true;
                                if self.view.ip_input_active {
                                    self.view.ip_input_active = false;
                                }
                                println!("Name input")
                            } else if name == "JOIN_GAME" {
                                new_view = Some(ViewType::Game(Map::new()));
                                println!("Game");
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
            };
            if let Some(view) = new_view {
                self.view = View::new(_ctx, view)?;
            }
        }

        Ok(())
    }

    fn text_input_event(&mut self, _ctx: &mut Context, character: char) -> Result<(), GameError> {
        println!("Character: {}", character);
        if self.view.ip_input_active && character.is_alphanumeric()
            || character == '.'
            || character == ':'
        {
            self.view.ip_address.add(character);
            println!("IP: {:?}", self.view.ip_address);
        }
        if self.view.name_input_active && character.is_alphanumeric() {
            self.view.name.add(character);
            println!("Name: {:?}", self.view.name);
        }
        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        input: ggez::input::keyboard::KeyInput,
        _repeated: bool,
    ) -> Result<(), GameError> {
        println!("Key: {:?}", input);

        if let Some(keycode) = input.keycode {
            if let keyboard::KeyCode::Back = keycode {
                if self.view.ip_input_active {
                    let mut new_string = self.view.ip_address.contents();
                    new_string.pop();
                    self.view.ip_address = Text::new(new_string);
                }
                if self.view.name_input_active {
                    let mut new_string = self.view.name.contents();
                    new_string.pop();
                    self.view.name = Text::new(new_string);
                }
            }
        }
        Ok(())
    }
}
