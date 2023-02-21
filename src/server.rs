use local_ip_address::local_ip;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    net::{SocketAddr, UdpSocket},
    str::FromStr,
    sync::mpsc::Sender,
};

#[derive(Serialize, Deserialize, Debug)]

pub enum Message {
    ClientJoined((String, String)),
    UpdateCounter(usize),
}
pub struct Server {
    pub socket: UdpSocket,
    pub clients: HashMap<String, String>,
}

impl Server {
    pub fn new() -> Server {
        let my_local_ip = local_ip().unwrap();
        Server {
            clients: HashMap::new(),
            socket: UdpSocket::bind(my_local_ip.to_string() + ":35353").unwrap(),
        }
    }

    pub fn start(&mut self) -> std::io::Result<()> {
        println!("Starting server...");
        println!("{:?}", self.socket);
        println!("");

        let mut buf = [0; 2048];

        loop {
            let (amt, src) = self.socket.recv_from(&mut buf)?;
            let m: Message = serde_json::from_slice(&buf[..amt]).unwrap();
            println!("SERVER: {:?}", m);

            match &m {
                Message::ClientJoined((name, ip_address)) => {
                    self.clients.insert(name.clone(), ip_address.clone());
                }
                Message::UpdateCounter(num) => {}
            };

            let m = serde_json::to_vec(&m).unwrap();
            for client in &self.clients {
                self.socket
                    .send_to(&m, SocketAddr::from_str(client.1).unwrap())?;

                // println!("SRC: {}", src);
                // println!("CLIENT IP: {}", client.1);
                // send game state
            }
            // let d = serde_json::to_vec(&data).unwrap();
            // let echo = std::str::from_utf8(&buf[..amt]).unwrap();
            // self.socket.send_to(&d, &src)?;
        }
    }
}
