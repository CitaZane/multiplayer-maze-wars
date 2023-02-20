use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    net::{SocketAddr, UdpSocket},
    str::FromStr,
    sync::mpsc::Sender,
};

use local_ip_address::local_ip;

#[derive(Serialize, Deserialize, Debug)]

pub enum Message {
    ClientJoined((String, String)),
}
pub struct Server {
    pub socket: UdpSocket,
    pub clients: HashMap<String, String>,
}

impl Server {
    pub fn new(ip_address: String) -> Server {
        Server {
            clients: HashMap::new(),
            socket: UdpSocket::bind(ip_address + ":34254").unwrap(),
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

pub fn connect_client(
    server_ip_address: String,
    name: String,
    send_ch: Sender<Message>,
) -> Result<(), std::io::Error> {
    let my_local_ip = local_ip().unwrap();
    let socket = UdpSocket::bind(my_local_ip.to_string() + ":0")?;

    let message = Message::ClientJoined((name, socket.local_addr().unwrap().to_string()));
    let message_bytes = serde_json::to_vec(&message).unwrap();
    socket
        .send_to(&message_bytes, server_ip_address.to_string() + ":34254")
        .expect("Error on send");

    let mut buf = [0; 2048];
    loop {
        let (amt, _) = socket.recv_from(&mut buf).expect("Didnt receive any data");
        let m: Message = serde_json::from_slice(&buf[..amt]).unwrap();

        match &m {
            Message::ClientJoined((name, ip_address)) => {
                println!("CLIENT: New user joined: {} {}", name, ip_address);
            }
        };

        send_ch.send(m).unwrap();

        // let echo = std::str::from_utf8(&buf[..amt]).unwrap();
        // println!("CLIENT: {:?}", d);
    }
}
