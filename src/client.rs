use std::{
    net::{SocketAddr, UdpSocket},
    str::FromStr,
    sync::mpsc::Sender,
};

use local_ip_address::local_ip;

use crate::server::Message;
pub struct Client {
    pub socket: UdpSocket,
    pub send_ch: Sender<Message>,
    pub name: String,
}

impl Client {
    pub fn new(send_ch: Sender<Message>, name: String) -> Client {
        let my_local_ip = local_ip().unwrap();
        Client {
            socket: UdpSocket::bind(my_local_ip.to_string() + ":0").unwrap(),
            send_ch,
            name,
        }
    }
    pub fn listen_for_messages(&self, server_ip_address: String) {
        let message = Message::ClientJoined((
            self.name.clone(),
            self.socket.local_addr().unwrap().to_string(),
        ));
        let message_bytes = serde_json::to_vec(&message).unwrap();
        self.socket
            .send_to(
                &message_bytes,
                SocketAddr::from_str(&server_ip_address).unwrap(),
            )
            .expect("Error on send");

        let mut buf = [0; 2048];
        loop {
            let (amt, _) = self
                .socket
                .recv_from(&mut buf)
                .expect("Didnt receive any data");
            let m: Message = serde_json::from_slice(&buf[..amt]).expect("Cant serialize from slice.");

            match &m {
                Message::ClientJoined((name, ip_address)) => {
                    println!("CLIENT: New user joined: {} {}", name, ip_address);
                }
                // Message::UpdateCounter(_num) => {}
                Message::PlayerMoved(name, cor, dir) => {
                    println!("Client: {:?}", &m)
                },
            };

            self.send_ch.send(m).unwrap();

            // let echo = std::str::from_utf8(&buf[..amt]).unwrap();
            // println!("CLIENT: {:?}", d);
        }
    }
}
