// use ggez::glam::Vec2;
use local_ip_address::local_ip;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    net::{SocketAddr, UdpSocket},
    str::FromStr,
};

#[derive(Serialize, Deserialize, Debug)]

pub enum Message {
    OpponentList(Vec<String>),
    PlayerShot((String, String)),   //Shooers name, opponents name
    ClientJoined((String, String)), // Name, ip-address
    PlayerMoved(String, (f32, f32), (f32, f32)), // Name, Coordinates(x,y), Direction (x, y)
    Map(Vec<Vec<i32>>),
    ConnectionLost,
    Ping,
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

    pub fn start(&mut self, maze: Vec<Vec<i32>>) -> std::io::Result<()> {
        println!("Starting server...");
        println!("Server IP: {:?}", self.socket.local_addr().unwrap());
        println!("");

        let mut buf = [0; 2048];

        loop {
            let (amt, src) = self.socket.recv_from(&mut buf)?;
            // let f = match self.socket.recv_from(&mut buf) {
            //     Ok(v) => v,
            //     Err(e) => {println!("Error: {e}")
            //     (0, SocketAddr::)
            // },
            // let f = self.socket.recv_from(&mut buf);

            let m: Message = serde_json::from_slice(&buf[..amt]).unwrap();
            // println!("SERVER: {:?}", m);

            match &m {
                Message::ClientJoined((name, ip_address)) => {
                    self.clients.insert(name.clone(), ip_address.clone());
                    self.send_user_list(name);
                    self.send_map(name, maze.clone());
                    self.send_to_all_clients(m);
                }
                Message::PlayerMoved(_, _, _) => {
                    self.send_to_all_clients(m);
                }
                Message::PlayerShot(_) => {
                    self.send_to_all_clients(m);
                }
                Message::Ping => {
                    // let s = 0;
                    println!("Got ping {}", src.to_string());
                    self.socket.send_to(&buf[..amt], src).unwrap();
                }

                _ => {}
            };
        }
    }
    fn send_map(&self, client: &String, maze: Vec<Vec<i32>>) {
        let msg = Message::Map(maze);
        let m = serde_json::to_vec(&msg).unwrap();
        let clien_socket = self.clients.get(client).unwrap();
        self.socket
            .send_to(
                &m,
                SocketAddr::from_str(&clien_socket).expect("Cant send data to all clients."),
            )
            .unwrap();
    }
    fn send_user_list(&self, client: &String) {
        // send message back to sender
        let list = Vec::from_iter(self.clients.keys())
            .iter()
            .map(|&value| value.to_owned())
            .collect();
        let msg = Message::OpponentList(list);
        let m = serde_json::to_vec(&msg).unwrap();
        let clien_socket = self.clients.get(client).unwrap();
        self.socket
            .send_to(
                &m,
                SocketAddr::from_str(&clien_socket).expect("Cant send data to all clients."),
            )
            .unwrap();
    }
    fn send_to_all_clients(&self, msg: Message) {
        let m = serde_json::to_vec(&msg).unwrap();
        for client in &self.clients {
            self.socket
                .send_to(
                    &m,
                    SocketAddr::from_str(client.1).expect("Cant send data to all clients."),
                )
                .unwrap();
        }
    }
}
