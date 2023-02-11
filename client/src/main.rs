use std::env;
use std::io::{self, BufRead};
use std::net::{SocketAddr, UdpSocket};
use std::str;
use ggez::{ContextBuilder, GameResult};
use ggez::event;
use ggez::conf::{WindowMode, Conf, WindowSetup};
mod game;
pub use game::*;
pub mod view;
pub mod map;
const SCREEN_WIDTH:f32 = 600.0;
const SCREEN_HEIGHT:f32= 800.0;

fn main()  -> GameResult {
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

fn connect() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage {} hostname", args[0]);
        std::process::exit(1);
    }
    let hostname = &args[1];

    let mut addrs: [SocketAddr; 20] = [SocketAddr::from(([0, 0, 0, 0], 3400)); 20];
    for i in 0..20{
        addrs[i] = SocketAddr::from(([0, 0, 0, 0], 3400 + i as u16));
    }
    let socket = UdpSocket::bind(&addrs[..]).expect("couldn't bind to address");

    // from https://stackoverflow.com/questions/30186037/how-can-i-read-a-single-line-from-stdin
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        println!("Line read from stdin '{}'", line);
        if &line == "BYE" {
            break;
        }

        socket
            .send_to(line.as_bytes(), hostname.to_string() + &":3500")
            .expect("Error on send");

        let mut buf = [0; 2048];
        let (amt, _src) = socket.recv_from(&mut buf)?;

        let _echo = str::from_utf8(&buf[..amt]).unwrap();
        // println!("Echo {}", echo);
    }
    Ok(())
}
