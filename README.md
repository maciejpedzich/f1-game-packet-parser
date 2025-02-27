# f1-game-packet-parser

This is a Rust crate that allows you to convert raw binary data from F1 24, F1 23, and F1 22 UDP telemetry into
organised structs.

## Getting started

Add `f1_game_packet_parser` to your project's `Cargo.toml` file:

```toml
[dependencies]
f1_game_packet_parser = "1.0.0"
```

## Example

This crate doesn't provide a UDP client out of the box. Here's how to write one that will parse and pretty-print
incoming packets:

```rust
use f1_game_packet_parser::parse;
use std::error::Error;
use std::net::UdpSocket;

fn main() -> Result<(), Box<dyn Error>> {
    // This IP and port should be set in the game's options by default.
    let socket = UdpSocket::bind("127.0.0.1:20777")?;
    let mut buf = [0u8; 1464];

    loop {
        // Receive raw packet data from the game.
        // The buf array should be large enough for all types of packets.
        let (amt, _) = socket.recv_from(&mut buf)?;

        // Convert received bytes to an F1Packet struct and print it.
        let packet = parse(&buf[..amt])?;
        println!("{:#?}", packet);
    }
}
```

## Minimum supported Rust version

The minimum supported Rust version is documented in the Cargo.toml file. This may be bumped in minor releases if
necessary.

## Original documentation links

- [F1 24](https://forums.ea.com/discussions/f1-24-general-discussion-en/f1-24-udp-specification/8369125)
- [F1 23](https://forums.ea.com/discussions/f1-23-en/f1-23-udp-specification/8390745)
- [F1 22](https://forums.ea.com/discussions/f1-games-franchise-discussion-en/f1-22-udp-specification/8418392)

## License

MIT
