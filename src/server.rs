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
    pub fn new(ip_address: String) -> Server {
        Server {
            clients: HashMap::new(),
            socket: UdpSocket::bind(ip_address + ":35353").unwrap(),
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

pub fn connect_client(
    client_socket: UdpSocket,
    name: String,
    server_ip_address: String,
    send_ch: Sender<Message>,
) -> Result<(), std::io::Error> {
    let message = Message::ClientJoined((name, client_socket.local_addr().unwrap().to_string()));
    let message_bytes = serde_json::to_vec(&message).unwrap();
    client_socket
        .send_to(
            &message_bytes,
            SocketAddr::from_str(&server_ip_address).unwrap(),
        )
        .expect("Error on send");

    let mut buf = [0; 2048];
    loop {
        let (amt, _) = client_socket
            .recv_from(&mut buf)
            .expect("Didnt receive any data");
        let m: Message = serde_json::from_slice(&buf[..amt]).unwrap();

        match &m {
            Message::ClientJoined((name, ip_address)) => {
                println!("CLIENT: New user joined: {} {}", name, ip_address);
            }
            Message::UpdateCounter(num) => {}
        };

        send_ch.send(m).unwrap();

        // let echo = std::str::from_utf8(&buf[..amt]).unwrap();
        // println!("CLIENT: {:?}", d);
    }
}
