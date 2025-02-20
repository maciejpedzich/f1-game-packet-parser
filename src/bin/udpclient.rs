use f1_game_packet_parser::parse;
use std::error::Error;
use std::net::UdpSocket;

fn main() -> Result<(), Box<dyn Error>> {
    let socket = UdpSocket::bind("127.0.0.1:20777")?;
    let mut buf = [0u8; 2048];

    loop {
        let (msg_size, _) = socket.recv_from(&mut buf)?;
        let data = &buf[..msg_size];

        parse(data)?;
    }
}
