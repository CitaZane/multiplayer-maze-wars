use local_ip_address::local_ip;
use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    let my_local_ip = local_ip().unwrap();
    let socket = UdpSocket::bind(my_local_ip.to_string() + ":34254")?; // for UDP4/6

    let mut buf = [0; 2048];
    println!("Server started at: {}", my_local_ip.to_string() + ":34254");
    loop {
        // Receives a single datagram message on the socket.
        // If `buf` is too small to hold
        // the message, it will be cut off.
        let (amt, src) = socket.recv_from(&mut buf)?;
        let echo = std::str::from_utf8(&buf[..amt]).unwrap();
        println!("Message: {}", echo);
        // Redeclare `buf` as slice of the received data
        // and send data back to origin.
        let buf = &mut buf[..amt];
        socket.send_to(buf, &src)?;
    }
}
