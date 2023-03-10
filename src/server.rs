use std::{time::{Duration, Instant}, sync::mpsc, thread};
use local_ip_address::local_ip;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    net::{SocketAddr, UdpSocket},
    str::FromStr,
};
use std::sync::mpsc::{ Receiver, Sender};

#[derive(Serialize, Deserialize, Debug)]

pub enum Message {
    OpponentList(Vec<String>),
    PlayerShot((String, String)),   //Shooters name, opponents name
    ClientJoined((String, String)), // Name, ip-address
    PlayerLeft(String),
    PlayerMoved(String, (f32, f32), (f32, f32)), // Name, Coordinates(x,y), Direction (x, y)
    Map(Vec<Vec<i32>>),
    ConnectionLost,
    Ping(String), //Client name
    Pong
}
pub struct Server {
    owner:String,
    pub socket: UdpSocket,
    pub clients: HashMap<String, (String,Instant)>,
    pub channels: (Sender<Message>, Receiver<Message>),
}

impl Server {
    pub fn new() -> Server {
        let my_local_ip = local_ip().unwrap();
        let server = Server {
            owner:String::new(),
            channels: mpsc::channel(),
            clients: HashMap::new(),
            socket: UdpSocket::bind(my_local_ip.to_string() + ":35353").unwrap(),
        };
        let send_ch = server.channels.0.clone();
        thread::spawn(move || loop{
            _ = send_ch.send(Message::Pong);
            thread::sleep(Duration::from_millis(1000))
        });
        server
    }
    pub fn start(&mut self, maze: Vec<Vec<i32>>) -> std::io::Result<()> {
        println!("Starting server...");
        println!("Server IP: {:?}", self.socket.local_addr().unwrap());
        println!("");

        let mut buf = [0; 2048];

        loop {
            // TEST PONG
            match self.channels.1.try_recv(){
                Ok(_)=>self.ping_pong_cleanup(),
                Err(_)=>{},
            };
            let (amt, src) = self.socket.recv_from(&mut buf)?;
            let m: Message = serde_json::from_slice(&buf[..amt]).unwrap();

            match &m {
                Message::ClientJoined((name, ip_address)) => {
                    if self.clients.len() ==0 {
                        self.owner = name.to_owned();
                    }
                    self.clients.insert(name.clone(),( ip_address.clone(), Instant::now()));
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
                Message::Ping(client_name) => {
                    self.socket.send_to(&buf[..amt], src).unwrap();
                    self.register_pong(client_name);
                }

                _ => {}
            };
        }
    }
    fn ping_pong_cleanup(&mut self){
        let mut remove_clinets = vec![];
        for (client,(_, time)) in &self.clients{
            let duration = time.elapsed();
            if duration > Duration::new(2,0) && *client != self.owner{
                remove_clinets.push(client.to_owned());
            }
        }
        for client in remove_clinets.iter(){
            println!("Remove client {}", client);
            self.clients.remove(client);
            self.send_to_all_clients(Message::PlayerLeft(client.to_owned()));
        }
    }
    fn register_pong(&mut self, client_name:&String){
        let  client =  self.clients.get_mut(client_name);
        if client.is_none(){return}
        client.unwrap().1=Instant::now();
    }
    fn send_map(&self, client: &String, maze: Vec<Vec<i32>>) {
        let msg = Message::Map(maze);
        let m = serde_json::to_vec(&msg).unwrap();
        let clien_socket = &self.clients.get(client).unwrap().0;
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
        let clien_socket = &self.clients.get(client).unwrap().0;
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
                    SocketAddr::from_str(&client.1.0).expect("Cant send data to all clients."),
                )
                .unwrap();
        }
    }
}
