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
    typing: bool,
    input: Text,
}
// 17 x 33
impl Game {
    pub fn new(_ctx: &mut Context) -> Self {
        Self {
            map: Map::new(),
            view: View::new(),
            input: Text::new(""),
            typing: false,
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

            match &self.view.current_view {
                ViewType::Game => todo!(),
                ViewType::MainMenu => {
                    for elem in &self.view.elements {
                        if x > elem.rect.x
                            && x < elem.rect.x + elem.rect.w
                            && y > elem.rect.y
                            && y < elem.rect.y + elem.rect.h
                        {
                            if elem.name == "CREATE_GAME" {
                                new_view = Some(ViewType::CreateGame);

                                println!("Create game");
                            } else if elem.name == "JOIN_GAME" {
                                new_view = Some(ViewType::JoinGame);
                                println!("Join game");
                            }
                        }
                    }
                }
                ViewType::JoinGame => {
                    for elem in &self.view.elements {
                        if x > elem.rect.x
                            && x < elem.rect.x + elem.rect.w
                            && y > elem.rect.y
                            && y < elem.rect.y + elem.rect.h
                        {
                            if elem.name == "IP_INPUT" {
                                println!("IP input");
                            } else if elem.name == "NAME_INPUT" {
                                println!("Name input")
                            } else if elem.name == "JOIN_GAME" {
                                new_view = Some(ViewType::Game);
                                println!("Game");
                            }
                        }
                    }
                    // name input
                    // ip input
                    // join game btn
                }
                ViewType::CreateGame => {
                    // name input
                    // create game btn
                    todo!()
                }
            };
            if let Some(view) = new_view {
                self.view.current_view = view;
            }
        }

        Ok(())
    }

    fn text_input_event(&mut self, _ctx: &mut Context, character: char) -> Result<(), GameError> {
        println!("Character: {}", character);
        if self.typing && character.is_alphanumeric() {
            if self.input.contents().len() < 10 {
                self.input.add(character);
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
        println!("Key: {:?}", input);

        if let Some(keycode) = input.keycode {
            if let keyboard::KeyCode::Back = keycode {
                let mut new_string = self.input.contents();
                new_string.pop();
                self.input = Text::new(new_string);
            }
        }
        Ok(())
    }
}
