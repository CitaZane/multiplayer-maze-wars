use std::env;
use std::io::{self, BufRead};
use std::net::{SocketAddr, UdpSocket};
use std::str;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage {} hostname", args[0]);
        std::process::exit(1);
    }
    let hostname = &args[1];

    let addrs = [
        SocketAddr::from(([0, 0, 0, 0], 3400)),
        SocketAddr::from(([0, 0, 0, 0], 3401)),
        SocketAddr::from(([0, 0, 0, 0], 3402)),
        SocketAddr::from(([0, 0, 0, 0], 3403)),
        SocketAddr::from(([0, 0, 0, 0], 3404)),
    ];
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

        let echo = str::from_utf8(&buf[..amt]).unwrap();
        // println!("Echo {}", echo);
    }
    Ok(())
}
