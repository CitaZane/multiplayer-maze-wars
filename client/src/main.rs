use ggez::conf::{Conf, WindowMode, WindowSetup};
use ggez::event;
use ggez::{ContextBuilder, GameResult};
use local_ip_address::local_ip;
use std::net::{SocketAddr, UdpSocket};
mod game;
pub use game::*;
pub mod map;
pub mod view;
const SCREEN_WIDTH: f32 = 600.0;
const SCREEN_HEIGHT: f32 = 800.0;

fn main() -> GameResult {
    // initialize socket connection for client
    let my_local_ip = local_ip().unwrap();
    _ = connect(my_local_ip.to_string());

    // Make a Context.
    let c = Conf::new();
    let window_mode = WindowMode::default().dimensions(SCREEN_WIDTH, SCREEN_HEIGHT);
    let window_setup = WindowSetup::default().title("Maze Wars");
    let (mut ctx, event_loop) = ContextBuilder::new("maze_wars", "The Gang")
        .default_conf(c)
        .window_setup(window_setup)
        .window_mode(window_mode)
        .add_resource_path("resources")
        .build()?;
    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let game = Game::new(&mut ctx)?;
    // Run!
    event::run(ctx, event_loop, game);
}

fn connect(ip_address: String) -> Result<UdpSocket, std::io::Error> {
    // "0" port will pick available one
    let socket = UdpSocket::bind(ip_address + ":0")?;

    // here we need to send to server address
    socket
        .send_to("client connected".as_bytes(), "192.168.1.126:34254")
        .expect("Error on send");

    // create buffer to save the socket message to
    let mut buf = [0; 2048];

    // load the message from the server to buffer and panic if any error happens
    socket.recv_from(&mut buf).expect("Didnt receive any data");

    Ok(socket)
}
