use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::time::Duration;
extern crate copypasta;

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
use copypasta::{ClipboardContext, ClipboardProvider};
pub struct State {
    // game_struct: GameStruct,
    pub view: View,
    pub server_ip: String,
    pub channels: (Sender<Message>, Receiver<Message>),
    pub client: Option<Arc<Client>>,
    pub map: Option<Map>,
    paste_ctx:ClipboardContext,
}

impl State {
    pub fn new(ctx: &mut Context) -> GameResult<State> {
        Ok(State {
            channels: mpsc::channel(),
            server_ip: String::new(),
            client: None,
            view: View::MainMenu(MainMenuStruct::new(ctx)?),
            map: None,
            paste_ctx:ClipboardContext::new().unwrap()
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
                    Message::PlayerShot(shot_data) => {
                        let got_shot = game.register_shooting(shot_data);
                        if got_shot {
                            let client = self.client.as_ref().unwrap();
                            let m = State::prepare_player_data_to_send(&client.name, &game.player);
                            client.socket.send_to(&m, self.server_ip.clone())?;
                        }
                    }
                    Message::Map(data) => {
                        game.map = Map::new(ctx, data);
                        let real_location = game.map.get_random_location();
                        game.player.pos.x = real_location.0;
                        game.player.pos.y = real_location.1;
                    }

                    Message::ConnectionLost => {
                        self.view = View::MainMenu(MainMenuStruct::new(ctx).unwrap());
                        return Ok(());
                    }

                    _ => {}
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
        if let View::JoinGame(view) = &mut self.view{
            if ctx.keyboard.is_key_just_pressed(keyboard::KeyCode::V) && ctx.keyboard.is_key_pressed(keyboard::KeyCode::LControl){
                view.paste_value(self.paste_ctx.get_contents().unwrap());
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
                    new_view = view_data.check_mouse_click(x, y, ctx);
                }
                View::CreateGame(view_data) => {
                    view_data.name_input_active = false;
                    new_view = view_data.check_mouse_click(x, y, ctx);
                }
                View::Game(_) => {}
                View::CreateMap(view_data) => {
                    view_data.name_input_active = false;
                    view_data.register_click(x, y, ctx);
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
                            let client = Arc::new(Client::new(name, server_ip.clone()));

                            self.client = Some(client.clone());
                            self.server_ip = server_ip.clone().to_string();

                            let client_clone = Arc::clone(&client);

                            let send_ch = self.channels.0.clone();
                            let send_ch1 = self.channels.0.clone();

                            let channels = channel::<bool>();
                            thread::spawn(move || loop {
                                if let Ok(_) = channels.1.try_recv() {
                                    println!("Lost connection to server...");
                                    send_ch.send(Message::ConnectionLost).unwrap();
                                    return;
                                };

                                client.send_ping_msg();
                                thread::sleep(Duration::from_millis(1000))
                            });

                            thread::spawn(move || {
                                client_clone.listen_for_messages(send_ch1);
                                channels.0.send(true).unwrap();
                            });
                        }
                        View::CreateGame(view_data) => {
                            let name = view_data.name.contents();
                            let send_ch = self.channels.0.clone();

                            let mut server = Server::new();
                            let server_ip =
                                server.socket.try_clone().unwrap().local_addr().unwrap();
                            // create client
                            let client = Arc::new(Client::new(name, server_ip.to_string().clone()));
                            let client_clone = Arc::clone(&client);

                            self.client = Some(client.clone());
                            self.map = Some(g.map.clone());
                            self.server_ip = server_ip.to_string();

                            let maze = self.map.as_ref().unwrap().maze.clone();
                            thread::spawn(move || server.start(maze).unwrap());
                            thread::spawn(move || client_clone.listen_for_messages(send_ch));
                        }
                        View::Game(_) => {}
                        View::MainMenu(_) => {}
                        View::CreateMap(_) => {}
                    }
                }
                self.view = view;
            }
        }
        if let MouseButton::Right = button{
            if let View::CreateMap(map) = &mut self.view{
                map.drag_mode_on();
                map.register_drag(x,y,ctx)
            }
        }

        Ok(())
    }
    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        button: MouseButton,
        _x: f32,
        _y: f32,
    ) -> Result<(), GameError>{
        if let MouseButton::Right = button{
            if let View::CreateMap(map) = &mut self.view{
                map.drag_mode_off();
            }
        }

        Ok(())
    }
    fn mouse_motion_event(
        &mut self,
        ctx: &mut Context,
        x: f32,
        y: f32,
        _dx:f32,
        _dy:f32
    ) -> Result<(), GameError>{
            if let View::CreateMap(map) = &mut self.view{
                if map.drag_mode{
                    map.register_drag(x,y,ctx)
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
