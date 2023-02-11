use ggez::conf::{Conf, WindowMode, WindowSetup};
use ggez::event;
use ggez::{ContextBuilder, GameResult};
use std::net::{SocketAddr, UdpSocket};
mod game;
pub use game::*;
pub mod map;
pub mod view;
const SCREEN_WIDTH: f32 = 600.0;
const SCREEN_HEIGHT: f32 = 800.0;

fn main() -> GameResult {
    // initialize socket connection for client
    let socket = connect();

    // Make a Context.
    let c = Conf::new();
    let window_mode = WindowMode::default().dimensions(SCREEN_WIDTH, SCREEN_HEIGHT);
    let window_setup = WindowSetup::default().title("Maze Wars");
    let (mut ctx, event_loop) = ContextBuilder::new("maze_wars", "The Gang")
        .default_conf(c)
        .window_setup(window_setup)
        .window_mode(window_mode)
        .build()?;
    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let game = Game::new(&mut ctx);
    // Run!
    event::run(ctx, event_loop, game);
}

fn connect() -> Result<UdpSocket, std::io::Error> {
    let mut addrs: [SocketAddr; 20] = [SocketAddr::from(([5, 5, 5, 5], 3400)); 20];
    // add 20 socket addresses to addrs with different ports
    for i in 0..20 {
        addrs[i] = SocketAddr::from(([0, 0, 0, 0], 3400 + i as u16));
    }

    // bind an address from addresses to socket.
    // it will pick one which is available
    let socket = UdpSocket::bind(&addrs[..])?;
    socket
        .send_to("client connected".as_bytes(), "127.0.0.1:3500")
        .expect("Error on send");

    // create buffer to save the socket message to
    let mut buf = [0; 2048];

    // load the message from the server to buffer and panic if any error happens
    socket.recv_from(&mut buf).unwrap();

    Ok(socket)
    // let args: Vec<String> = env::args().collect();
    // if args.len() < 2 {
    //     println!("Usage {} hostname", args[0]);
    //     std::process::exit(1);
    // }
    // let hostname = &args[1];
    // from https://stackoverflow.com/questions/30186037/how-can-i-read-a-single-line-from-stdin
    // let stdin = io::stdin();
    // for line in stdin.lock().lines() {
    //     let line = line.unwrap();
    //     println!("Line read from stdin '{}'", line);
    //     if &line == "BYE" {
    //         break;
    //     }

    //     socket
    //         .send_to(line.as_bytes(), hostname.to_string() + &":3500")
    //         .expect("Error on send");

    //     let mut buf = [0; 2048];
    //     let (amt, _src) = socket.recv_from(&mut buf)?;

    //     let _echo = str::from_utf8(&buf[..amt]).unwrap();
    //     // println!("Echo {}", echo);
    // }
    // Ok(())
}
