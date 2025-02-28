# f1-game-packet-parser

This is a Rust crate that allows you to convert binary data from F1 24, F1 23, and F1 22 UDP telemetry into organised structs.

## Getting started

Add `f1-game-packet-parser` to your project by running this command:

```
cargo add f1-game-packet-parser
```

## Example

This crate doesn't provide a UDP client out of the box. Here's how to write one that will keep parsing incoming packets until it receives final classification or a _session ended_ event packet. It will then print either the final classification or a _Session has ended!_ message and exit.

```rust
use f1_game_packet_parser::packets::event::EventDetails;
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

        // Convert received bytes to an F1Packet struct.
        let packet = parse(&buf[..amt])?;

        // It's the final classification confirmation.
        if let Some(final_classification) = packet.final_classification {
            println!("{:#?}", final_classification);
            break;
        }
        // It's the session ended event.
        else if packet
            .event
            .is_some_and(|event| event.details == EventDetails::SessionEnded)
        {
            println!("Session has ended!");
            break;
        }
    }
    
    Ok(())
}
```

## Minimum supported Rust version

The minimum supported Rust version is documented in the `Cargo.toml` file. It may be bumped in minor releases if necessary.

## FAQ

### What about support for F1 2021-2018?

My initial goal was to provide support from the most recent game all the way to F1 2018, starting development with implementing support for the latter and _working my way up_. I ended up limiting my crate's backwards compatibility to F1 22, because:
- When I was done with the 2021 format, certain structs had so many fields that got added, removed, or in some cases even reordered with each new format that they became a nightmare to manage and navigate through in the documentation.
- I realised that most users would probably never end up using formats that are over 2-3 years old anyway. Limiting support to the 3 most recent packet formats also happens to be the actual policy in the latest games.

However, if you believe there's a method in the madness and/or need support for these older packet formats, feel free to open an issue or a pull request.

### Are there any alternatives to this crate?

If you're looking for a crate that supports an older telemetry format or need a built-in UDP client, consider one of the following:

+ [f1_game_telemetry](https://crates.io/crates/f1_game_telemetry) - supports the 2022 format and offers a built-in UDP client
+ [f1-telemetry-client](https://crates.io/crates/f1-telemetry-client) - supports the 2020 format and offers a built-in UDP client

### Where can I find the links to the original specification documents?

In the list below:

- [F1 24](https://forums.ea.com/discussions/f1-24-general-discussion-en/f1-24-udp-specification/8369125)
- [F1 23](https://forums.ea.com/discussions/f1-23-en/f1-23-udp-specification/8390745)
- [F1 22](https://forums.ea.com/discussions/f1-games-franchise-discussion-en/f1-22-udp-specification/8418392)

## License

MIT
