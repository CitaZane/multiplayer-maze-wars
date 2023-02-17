use std::{
    collections::HashMap,
    net::{IpAddr, UdpSocket},
};

use local_ip_address::local_ip;

pub struct Server {
    ip_address: IpAddr,
    clients: HashMap<String, UdpSocket>,
}

impl Server {
    pub fn new(ip_address: IpAddr) -> Server {
        Server {
            ip_address,
            clients: HashMap::new(),
        }
    }

    pub fn connect_client(&mut self, name: String) -> Result<(), std::io::Error> {
        let my_local_ip = local_ip().unwrap();
        let socket = UdpSocket::bind(my_local_ip.to_string() + ":0")?;

        // here we need to send to server address
        socket
            .send_to(
                format!("{} connected", name).as_bytes(),
                self.ip_address.to_string() + ":34254",
            )
            .expect("Error on send");

        // create buffer to save the socket message to
        let mut buf = [0; 2048];

        // load the message from the server to buffer and panic if any error happens
        socket.recv_from(&mut buf).expect("Didnt receive any data");
        self.clients.insert(name, socket);
        Ok(())
    }

    pub fn start(&self) -> std::io::Result<()> {
        let socket = UdpSocket::bind(self.ip_address.to_string() + ":34254")?; // for UDP4/6

        let mut buf = [0; 2048];
        println!(
            "Server started at: {}",
            self.ip_address.to_string() + ":34254"
        );
        loop {
            let (amt, src) = socket.recv_from(&mut buf)?;
            let echo = std::str::from_utf8(&buf[..amt]).unwrap();
            println!("Message: {}", echo);
            // Redeclare `buf` as slice of the received data
            // and send data back to origin.
            let buf = &mut buf[..amt];
            socket.send_to(buf, &src)?;
        }
    }
}
