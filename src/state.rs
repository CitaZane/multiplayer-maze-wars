use std::net::UdpSocket;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

use crate::client::Client;
use crate::main_menu::MainMenuStruct;
pub use crate::map::Map;
pub use crate::player::Player;
use crate::server::{Message, Server};
use crate::view::{remove_input_text_last_letter, View};
// pub use crate::view::View;
use ggez::event::{EventHandler, MouseButton};
use ggez::graphics::{self, Color};
use ggez::input::keyboard::{self, KeyCode};
use ggez::{Context, GameError, GameResult};
use std::sync::mpsc;
pub struct State {
    pub view: View,
    pub client_socket: Option<UdpSocket>,
    pub server_ip: String,
    pub counter: usize,
    pub channels: (Sender<Message>, Receiver<Message>),
}

impl State {
    pub fn new(ctx: &mut Context) -> GameResult<State> {
        Ok(State {
            channels: mpsc::channel(),
            counter: 0,
            server_ip: String::new(),
            client_socket: None,
            view: View::MainMenu(MainMenuStruct::new(ctx)?),
        })
    }
}

impl EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if let View::Game(game_data) = &mut self.view {
            if let Ok(msg) = self.channels.1.try_recv() {
                println!("MAIN THREAD: {:?}", msg);
            }

            if ctx.keyboard.is_key_pressed(KeyCode::Up) || ctx.keyboard.is_key_pressed(KeyCode::W) {
                game_data.player.go_forward(&game_data.map.maze);
                self.counter += 1;
                // println!("server socket: {:?}", self.server_ip);
                let m = serde_json::to_vec(&Message::UpdateCounter(self.counter)).unwrap();
                self.client_socket
                    .as_ref()
                    .unwrap()
                    .send_to(&m, self.server_ip.clone())?;
            }
            if ctx.keyboard.is_key_pressed(KeyCode::Down) || ctx.keyboard.is_key_pressed(KeyCode::S)
            {
                game_data.player.go_backward(&game_data.map.maze);
            }
            if ctx.keyboard.is_key_pressed(KeyCode::Left) || ctx.keyboard.is_key_pressed(KeyCode::A)
            {
                game_data.player.turn_left();
            }
            if ctx.keyboard.is_key_pressed(KeyCode::Right)
                || ctx.keyboard.is_key_pressed(KeyCode::D)
            {
                game_data.player.turn_right();
            }
        }
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
                    new_view = view_data.check_mouse_click(x, y, ctx, self.channels.0.clone());
                }
                View::CreateGame(view_data) => {
                    view_data.name_input_active = false;
                    new_view = view_data.check_mouse_click(x, y, ctx, self.channels.0.clone());
                }
                View::Game(_) => {}
            };

            if let Some(view) = new_view {
                if let View::Game(_) = &view {
                    let previous_view = &self.view;
                    // if create game was previously -> create server
                    // if join game was previously -> connect to server
                    match previous_view {
                        View::JoinGame(view_data) => {
                            let name = view_data.name.contents();
                            let server_ip = view_data.ip_address.contents() + ":35353";
                            let send_ch = self.channels.0.clone();
                            let client = Client::new(send_ch, name);

                            self.client_socket = Some(client.socket.try_clone().unwrap());
                            self.server_ip = server_ip.to_string();

                            thread::spawn(move || client.listen_for_messages(server_ip));
                        }
                        View::CreateGame(view_data) => {
                            let name = view_data.name.contents();
                            let send_ch = self.channels.0.clone();

                            // create client
                            let client = Client::new(send_ch, name);

                            let mut server = Server::new();
                            let server_ip =
                                server.socket.try_clone().unwrap().local_addr().unwrap();

                            self.client_socket = Some(client.socket.try_clone().unwrap());
                            self.server_ip = server_ip.to_string();

                            thread::spawn(move || server.start().unwrap());
                            thread::spawn(move || {
                                client.listen_for_messages(server_ip.to_string())
                            });
                        }
                        View::Game(_) => {}
                        View::MainMenu(_) => {}
                    }
                }
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
