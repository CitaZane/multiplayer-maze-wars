use std::{
    net::{SocketAddr, UdpSocket},
    str::FromStr,
    sync::{
        mpsc::{Sender},
    },
};

use local_ip_address::local_ip;

use crate::server::Message;

pub struct Client {
    pub socket: UdpSocket,
    pub name: String,
    pub server_ip: String,
}

impl Client {
    pub fn new(name: String, server_ip: String) -> Client {
        let my_local_ip = local_ip().unwrap();
        Client {
            socket: UdpSocket::bind(my_local_ip.to_string() + ":0").unwrap(),
            name,
            server_ip,
        }
    }
    pub fn listen_for_messages(&self, send_ch: Sender<Message>) {
        let message = Message::ClientJoined((
            self.name.clone(),
            self.socket.local_addr().unwrap().to_string(),
        ));
        let message_bytes = serde_json::to_vec(&message).unwrap();
        self.socket
            .send_to(
                &message_bytes,
                SocketAddr::from_str(&self.server_ip).unwrap(),
            )
            .expect("Error on send");

        let mut buf = [0; 2048];
        loop {
            match self.socket.recv_from(&mut buf) {
                Ok((amt, _)) => {
                    let m: Message =
                        serde_json::from_slice(&buf[..amt]).expect("Cant serialize from slice.");

                    match &m {
                        Message::ClientJoined((name, ip_address)) => {
                            println!("CLIENT: New user joined: {} {}", name, ip_address);
                        }
                        Message::Ping(_) => {
                            // println!("Got ping back")
                        }
                        _ => {}
                    };
                    send_ch.send(m).unwrap();
                }

                Err(_e) => {
                    return;
                }
            }
        }
    }

    pub fn send_ping_msg(&self) {
        let m = serde_json::to_vec(&Message::Ping(self.name.clone())).unwrap();
        self.socket.send_to(&m, &self.server_ip).unwrap();
    }
}
