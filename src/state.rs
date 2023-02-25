use std::sync::mpsc::{Receiver, Sender};
use std::thread;

use crate::client::Client;

use crate::main_menu::MainMenuStruct;
pub use crate::map::Map;
use crate::player::Direction;
pub use crate::player::Player;
use crate::server::{Message, Server};
use crate::view::{remove_input_text_last_letter, View};
use ggez::event::{EventHandler, MouseButton};
use ggez::graphics::{self, Color};
use ggez::input::keyboard::{self, KeyCode};
use ggez::{Context, GameError, GameResult};
use std::sync::{mpsc, Arc};
pub struct State {
    // game_struct: GameStruct,
    pub view: View,
    pub server_ip: String,
    pub channels: (Sender<Message>, Receiver<Message>),
    pub client: Option<Arc<Client>>,
    pub map:Option<Map>,
}

impl State {
    pub fn new(ctx: &mut Context) -> GameResult<State> {
        Ok(State {
            channels: mpsc::channel(),
            server_ip: String::new(),
            client: None,
            view: View::MainMenu(MainMenuStruct::new(ctx)?),
            map:None,
        })
    }
    fn prepare_player_data_to_send(player_name: &String, player_data: &Player) -> Vec<u8> {
        serde_json::to_vec(&Message::PlayerMoved(
            player_name.clone(),
            (player_data.pos.x, player_data.pos.y),
            (player_data.dir.vec().x, player_data.dir.vec().y),
        ))
        .expect("Cant disserialize.")
    }
    fn prepare_shoot_data_to_send(player_name: String, opponent_name: String) -> Vec<u8> {
        serde_json::to_vec(&Message::PlayerShot((player_name, opponent_name)))
            .expect("Cant disserialize.")
    }
}

impl EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if let View::Game(game) = &mut self.view {
            if let Ok(msg) = self.channels.1.try_recv() {
                match msg {
                    Message::ClientJoined(msg) => {
                        if msg.0 != game.player.name {
                            game.add_opponents(vec![msg.0]);
                        }
                    }
                    Message::PlayerMoved(name, cor, dir) => {
                        for opponent in game.opponents.iter_mut() {
                            if opponent.name == name {
                                opponent.pos.x = cor.0;
                                opponent.pos.y = cor.1;
                                opponent.dir = Direction::from_vec(&dir);
                            }
                        }
                    }
                    Message::OpponentList(list) => game.add_opponents(list),
                    Message::PlayerShot(shot_data) => game.register_shooting(shot_data),
                    Message::Map(data)=>{game.map = Map::new(ctx, data)}
                }
            }
            if !ctx.keyboard.is_key_pressed(KeyCode::Space) {
                game.player.can_shoot = true;
            }
            if ctx.keyboard.is_key_pressed(KeyCode::Up) || ctx.keyboard.is_key_pressed(KeyCode::W) {
                if game.player.go_forward(&game.map.maze) {
                    let client = self.client.as_ref().unwrap();
                    let m = State::prepare_player_data_to_send(&client.name, &game.player);
                    client.socket.send_to(&m, self.server_ip.clone())?;
                }
            }
            if ctx.keyboard.is_key_pressed(KeyCode::Down) || ctx.keyboard.is_key_pressed(KeyCode::S)
            {
                if game.player.go_backward(&game.map.maze) {
                    let client = self.client.as_ref().unwrap();
                    let m = State::prepare_player_data_to_send(&client.name, &game.player);
                    client.socket.send_to(&m, self.server_ip.clone())?;
                }
            }
            if ctx.keyboard.is_key_pressed(KeyCode::Left) || ctx.keyboard.is_key_pressed(KeyCode::A)
            {
                if game.player.turn_left() {
                    let client = self.client.as_ref().unwrap();
                    let m = State::prepare_player_data_to_send(&client.name, &game.player);
                    client.socket.send_to(&m, self.server_ip.clone())?;
                }
            }
            if ctx.keyboard.is_key_pressed(KeyCode::Right)
                || ctx.keyboard.is_key_pressed(KeyCode::D)
            {
                if game.player.turn_right() {
                    let client = self.client.as_ref().unwrap();
                    let m = State::prepare_player_data_to_send(&client.name, &game.player);
                    client.socket.send_to(&m, self.server_ip.clone())?;
                }
            }
            if ctx.keyboard.is_key_pressed(KeyCode::Space) {
                if game.player.can_shoot {                    
                    let shot = game.shoot(ctx);
                    if shot.is_some() {
                        let (shooter, target) = shot.unwrap();
                        let client = self.client.as_ref().unwrap();
                        let m = State::prepare_shoot_data_to_send(shooter, target);
                        client.socket.send_to(&m, self.server_ip.clone())?;
                    }
                    game.player.can_shoot = false;
                }                
            }
            game.update()?;
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
                    new_view = view_data.check_mouse_click(x, y, ctx);
                }
                View::CreateGame(view_data) => {
                    view_data.name_input_active = false;
                    new_view = view_data.check_mouse_click(x, y, ctx);
                }
                View::Game(_) => {}
                View::CreateMap(view_data) => {
                    view_data.name_input_active = false;
                    view_data.register_click(x, y,ctx);
                    new_view = view_data.check_mouse_click(x, y, ctx);
                }
            };

            if let Some(view) = new_view {
                if let View::Game(g) = &view {
                    let previous_view = &self.view;
                    // if create game was previously -> create server
                    // if join game was previously -> connect to server
                    match previous_view {
                        View::JoinGame(view_data) => {
                            let name = view_data.name.contents();
                            let server_ip = view_data.ip_address.contents() + ":35353";

                            let client = Arc::new(Client::new(name));
                            let client_clone = Arc::clone(&client);
                            let send_ch = self.channels.0.clone();

                            self.client = Some(client.clone());
                            self.server_ip = server_ip.to_string();
                            thread::spawn(move || {
                                client_clone.listen_for_messages(server_ip, send_ch)
                            });
                        }
                        View::CreateGame(view_data) => {
                            let name = view_data.name.contents();
                            let send_ch = self.channels.0.clone();

                            // create client
                            let client = Arc::new(Client::new(name));
                            let client_clone = Arc::clone(&client);
                            self.client = Some(client.clone());

                            let mut server = Server::new();
                            let server_ip =
                                server.socket.try_clone().unwrap().local_addr().unwrap();
                            self.map = Some(g.map.clone());
                            self.server_ip = server_ip.to_string();
                            let maze = self.map.as_ref().unwrap().maze.clone();
                            thread::spawn(move || server.start(maze).unwrap());
                            thread::spawn(move || {
                                client_clone.listen_for_messages(server_ip.to_string(), send_ch)
                            });
                        }
                        View::Game(_) => {}
                        View::MainMenu(_) => {}
                        View::CreateMap(_) => {}
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
            View::CreateMap(view_data) => {
                if view_data.name_input_active
                    && character.is_alphanumeric()
                    && view_data.name.contents().len() <= 10
                {
                    view_data.name.add(character);
                }
            }
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
                    View::CreateMap(view_data) => {
                        if view_data.name_input_active {
                            view_data.name =
                                remove_input_text_last_letter(view_data.name.contents());
                        }
                    }
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
