use f1_game_packet_parser::parse;
use std::error::Error;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::net::UdpSocket;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn Error>> {
    let addr = "127.0.0.1:20777";
    let socket = UdpSocket::bind(addr)?;
    let mut buf = [0u8; 1464];

    println!("UDP client is listening on {}", addr);

    loop {
        let (amt, _) = socket.recv_from(&mut buf)?;

        let raw_data = &buf[..amt];
        let packet = parse(raw_data)?;
        let file_path = PathBuf::from(format!(
            "tests/data/{}-{:0>2}{}.bin",
            packet.header.packet_format,
            packet.header.packet_id as u8,
            if let Some(event) = &packet.event {
                format!("-{}", event.code)
            } else {
                "".to_string()
            }
        ));

        if !file_path.exists() {
            let bin_file = File::create(file_path)?;
            let mut writer = BufWriter::new(bin_file);

            writer.write_all(raw_data)?;
            writer.flush()?;
        }
    }
}
