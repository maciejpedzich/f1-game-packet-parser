use f1_game_packet_parser::parse;
use std::error::Error;
use std::net::UdpSocket;

fn main() -> Result<(), Box<dyn Error>> {
    let addr = "127.0.0.1:20777";
    let socket = UdpSocket::bind(addr)?;
    let mut buf = [0u8; 2048];

    println!("UDP client is listening on {}", addr);

    loop {
        let (msg_len, _) = socket.recv_from(&mut buf)?;
        let msg = &buf[..msg_len];

        parse(msg)?;
    }
}
