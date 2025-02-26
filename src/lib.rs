//! Convert raw binary data from F1 24, F1 23, and F1 22 UDP telemetry into organised structs.
//! ## Getting started
//!
//! Add `f1_game_packet_parser` to your project's `Cargo.toml` file:
//!
//! ```toml
//! [dependencies]
//! f1_game_packet_parser = "1.0.0"
//! ```
//!
//! ### Basic UDP client
//!
//! This crate doesn't provide a UDP client out of the box.
//! Here's how to write one that will parse and pretty-print incoming packets:
//!
//! ```no_run
//! use f1_game_packet_parser::parse;
//! use std::error::Error;
//! use std::net::UdpSocket;
//!
//! fn main() -> Result<(), Box<dyn Error>> {
//!     // This IP and port should be set in the game's options by default.
//!     let socket = UdpSocket::bind("127.0.0.1:20777")?;
//!     let mut buf = [0u8; 1464];
//!
//!     loop {
//!         // Receive raw packet data from the game.
//!         // The buf array should be large enough for all types of packets.
//!         let (amt, _) = socket.recv_from(&mut buf)?;
//!
//!         // Convert received bytes to an F1Packet struct and print it.
//!         let packet = parse(&buf[..amt])?;
//!         println!("{:#?}", packet);
//!     }
//! }
//! ```
//!
//! ### Determining a packet's type and extracting its payload
//!
//! An [`F1Packet`] consists of a universal [`header`](field@F1Packet::header)
//! and an [`Option`] field for a payload of every single packet type.
//! Only one of these can be set to [`Some`] for a given [`F1Packet`] instance.
//!
//! Therefore, you can use the following if-else-if chain to
//! differentiate between all packet types and extract their payloads.
//! Of course, you can remove the branches you don't need.
//!
//! ```no_run
//! use f1_game_packet_parser::parse;
//!
//! let placeholder_data = include_bytes!("placeholder.bin");
//! let packet = parse(placeholder_data)?;
//!
//! if let Some(motion) = &packet.motion {
//!     // Do whatever with motion.
//! } else if let Some(session) = &packet.session {
//!     // Do whatever with session.
//! } else if let Some(laps) = &packet.laps {
//!     // Do whatever with laps.
//! } else if let Some(event) = &packet.event {
//!     // Do whatever with event.
//! } else if let Some(participants) = &packet.participants {
//!     // Do whatever with participants.
//! } else if let Some(car_setups) = &packet.car_setups {
//!     // Do whatever with car_setups.
//! } else if let Some(car_telemetry) = &packet.car_telemetry {
//!     // Do whatever with car_telemetry.
//! } else if let Some(car_status) = &packet.car_status {
//!     // Do whatever with car_status.
//! } else if let Some(final_classification) = &packet.final_classification {
//!     // Do whatever with final_classification.
//! } else if let Some(lobby) = &packet.lobby {
//!     // Do whatever with lobby.
//! } else if let Some(car_damage) = &packet.car_damage {
//!     // Do whatever with car_damage.
//! } else if let Some(session_history) = &packet.session_history {
//!     // Do whatever with session_history.
//! } else if let Some(tyre_sets) = &packet.tyre_sets {
//!     // Available from the 2023 format onwards.
//!     // Do whatever with tyre_sets.
//! } else if let Some(motion_ex) = &packet.motion_ex {
//!     // Available from the 2023 format onwards.
//!     // Do whatever with motion_ex.
//! } else if let Some(time_trial) = &packet.time_trial {
//!     // Available from the 2024 format onwards.
//!     // Do whatever with time_trial.
//! }
//! ```
//!
//! ### Working with [event packets](F1PacketEvent)
//!
//! [`F1PacketEvent`] is unique among other kinds of packets.
//! Its payload consists of a 4-letter code that determines
//! the type of the event, followed by optional details
//! about this event.
//!
//! These extra details are represented by the [`EventDetails`](enum@packets::event::EventDetails)
//! enum (even if a certain event doesn't come with additional data).
//! You can import the enum and use a matcher to determine an event's type
//! and extract its payload (if available) like so:
//!
//! ```no_run
//! use f1_game_packet_parser::packets::event::EventDetails;
//! use f1_game_packet_parser::parse;
//!
//! let placeholder_data = include_bytes!("placeholder.bin");
//! let packet = parse(placeholder_data)?;
//!
//! if let Some(event) = &packet.event {
//!     match event.details {
//!         /// Event with no extra details.
//!         EventDetails::LightsOut => {
//!             println!("It's lights out, and away we go!");
//!         }
//!         /// You can skip the details if you don't need them.
//!         EventDetails::Flashback { .. } => {
//!             println!("Flashback has been triggered!");
//!         }
//!         /// Extracting details from an event.
//!         EventDetails::RaceWinner { vehicle_index } => {
//!             println!(
//!                 "Driver at index {} is the winner!",
//!                 vehicle_index
//!             );
//!         }
//!         _ => (),
//!     }
//! }
//! ```
//!
//! ### Working with bitmaps
//!
//! There are 3 fields that use a [`bitflags`]-powered bitmap struct:
//!
//! - [`EventDetails::Buttons::button_status`](field@packets::event::EventDetails::Buttons::button_status)
//! - [`CarTelemetryData::rev_lights_bit_value`](field@packets::car_telemetry::CarTelemetryData::rev_lights_bit_value)
//! - [`LapHistoryData::lap_valid_bit_flags`](field@packets::session_history::LapHistoryData::lap_valid_bit_flags)
//!
//! Each bitmap struct is publicly available via the [`constants`] module
//! and comes with a handful of constants representing specific bit values,
//! as well as methods and operator overloads for common bit operations.
//!
//! Here's an example that checks if a given binary file is
//! a [car telemetry packet](F1PacketCarTelemetry).
//! If so, it will grab player car's telemetry data
//! and determine whether the revs are high, medium or low
//! based on the specific bit values being set.
//!
//! ```no_run
//! use f1_game_packet_parser::constants::RevLights;
//! use f1_game_packet_parser::parse;
//!
//! let placeholder_data = include_bytes!("placeholder.bin");
//! let packet = parse(placeholder_data)?;
//! let player_car_index = packet.header.player_car_index;
//!
//! if let Some(car_telemetry) = &packet.car_telemetry {
//!     let player = car_telemetry.data[player_car_index];
//!     let is_high_rev =
//!         player.rev_lights_bit_value.contains(RevLights::RIGHT_1);
//!     let is_medium_rev =
//!         player.rev_lights_bit_value.contains(RevLights::MIDDLE_1);
//!
//!     let revs_desc = if is_high_rev {
//!         "High"
//!     } else if is_medium_rev {
//!         "Medium"
//!     } else {
//!         "Low"
//!     };
//!
//!     println!("{} revs", revs_desc);
//! }
//! ```
//!
//! ## Original documentation links
//!
//! - [F1 24](https://forums.ea.com/discussions/f1-24-general-discussion-en/f1-24-udp-specification/8369125)
//! - [F1 23](https://forums.ea.com/discussions/f1-23-en/f1-23-udp-specification/8390745)
//! - [F1 22](https://forums.ea.com/discussions/f1-games-franchise-discussion-en/f1-22-udp-specification/8418392)

/// Contains appendix constants and enums for various packet-specific struct field values.
pub mod constants;
/// Contains structures for each kind of packet payload
/// and submodules for packet-specific structs.
pub mod packets;

use crate::constants::{PacketId, MAX_NUM_CARS};
use crate::packets::{
    u8_to_usize, F1PacketCarDamage, F1PacketCarSetups, F1PacketCarStatus,
    F1PacketCarTelemetry, F1PacketEvent, F1PacketFinalClassification, F1PacketLaps,
    F1PacketLobby, F1PacketMotion, F1PacketMotionEx, F1PacketParticipants,
    F1PacketSession, F1PacketSessionHistory, F1PacketTimeTrial, F1PacketTyreSets,
};

use binrw::io::Cursor;
use binrw::{BinRead, BinReaderExt, BinResult};
use serde::{Deserialize, Serialize};

/// Attempts to extract F1 game packet data from a shared slice of bytes.
///
/// ## Errors
///
/// - [`binrw::Error::AssertFail`] when a certain field's value
///   is outside the expected range. This generally applies to
///   `_index` fields, percentage values, and fields that
///   have the aforementioned range specified in their documentation
/// - [`binrw::Error::BadMagic`] when [`F1PacketEvent`] has a code
///   that doesn't match any [known event type](packets::event::EventDetails)
/// - [`binrw::Error::Custom`] when the parser encounters an invalid bool value
///   in a read byte (i.e. neither 0, nor 1)
/// - [`binrw::Error::EnumErrors`] when there's no matching value for an enum field
///
/// ## Examples
///
/// ### Basic UDP client
///
/// ```no_run
/// use f1_game_packet_parser::parse;
/// use std::error::Error;
/// use std::net::UdpSocket;
///
/// fn main() -> Result<(), Box<dyn Error>> {
///     // This IP and port should be set in the game's options by default.
///     let socket = UdpSocket::bind("127.0.0.1:20777")?;
///     let mut buf = [0u8; 1464];
///
///     loop {
///         // Receive raw packet data from the game.
///         // The buf array should be large enough for all types of packets.
///         let (amt, _) = socket.recv_from(&mut buf)?;
///
///         // Convert received bytes to an F1Packet struct and print it.
///         let packet = parse(&buf[..amt])?;
///         println!("{:#?}", packet);
///     }
/// }
/// ```
///
/// ### Invalid/unsupported packet format
///
/// ```
/// let invalid_format = 2137u16.to_le_bytes();
/// let parse_result = f1_game_packet_parser::parse(&invalid_format);
///
/// assert!(parse_result.is_err());
/// assert_eq!(
///     parse_result.unwrap_err().to_string(),
///     "Invalid or unsupported packet format: 2137 at 0x0"
/// );
/// ```
pub fn parse(data: &[u8]) -> BinResult<F1Packet> {
    let mut cursor = Cursor::new(data);
    let packet: F1Packet = cursor.read_le()?;

    Ok(packet)
}

/// Structured representation of raw F1 game packet data that's
/// returned as a successful result of the [`parse`] function.
///
/// Each [`Option`] field acts as a slot for a payload of a packet of a certain type.
/// Only one of these fields can be [`Some`] for a given `F1Packet` instance.
#[derive(BinRead, PartialEq, PartialOrd, Clone, Debug, Serialize, Deserialize)]
#[br(little)]
pub struct F1Packet {
    /// Universal packet header.
    pub header: F1PacketHeader,
    /// Physics data for all cars in the ongoing session.
    #[br(if(header.packet_id == PacketId::Motion), args(header.packet_format))]
    pub motion: Option<F1PacketMotion>,
    /// Data about the ongoing session.
    #[br(if(header.packet_id == PacketId::Session), args(header.packet_format))]
    pub session: Option<F1PacketSession>,
    /// Lap data for all cars on track.
    #[br(if(header.packet_id == PacketId::Laps), args(header.packet_format))]
    pub laps: Option<F1PacketLaps>,
    /// Details of events that happen during the course of the ongoing session.
    #[br(if(header.packet_id == PacketId::Event), args(header.packet_format))]
    pub event: Option<F1PacketEvent>,
    /// List of participants in the session.
    #[br(if(header.packet_id == PacketId::Participants), args(header.packet_format))]
    pub participants: Option<F1PacketParticipants>,
    /// Setup data for all cars in the ongoing session.
    #[br(if(header.packet_id == PacketId::CarSetups), args(header.packet_format))]
    pub car_setups: Option<F1PacketCarSetups>,
    /// Telemetry data for all cars in the ongoing session.
    #[br(if(header.packet_id == PacketId::CarTelemetry), args(header.packet_format))]
    pub car_telemetry: Option<F1PacketCarTelemetry>,
    /// Status data for all cars in the ongoing session.
    #[br(if(header.packet_id == PacketId::CarStatus), args(header.packet_format))]
    pub car_status: Option<F1PacketCarStatus>,
    /// Final classification confirmation at the end of the session.
    #[br(
        if(header.packet_id == PacketId::FinalClassification),
        args(header.packet_format)
    )]
    pub final_classification: Option<F1PacketFinalClassification>,
    /// Details of players in a multiplayer lobby.
    #[br(if(header.packet_id == PacketId::LobbyInfo), args(header.packet_format))]
    pub lobby: Option<F1PacketLobby>,
    /// Car damage parameters for all cars in the ongoing session.
    #[br(if(header.packet_id == PacketId::CarDamage), args(header.packet_format))]
    pub car_damage: Option<F1PacketCarDamage>,
    /// Session history data for a specific car.
    #[br(if(header.packet_id == PacketId::SessionHistory), args(header.packet_format))]
    pub session_history: Option<F1PacketSessionHistory>,
    /// In-depth details about tyre sets assigned to a vehicle during the session.
    /// Available from the 2023 format onwards.
    #[br(if(header.packet_id == PacketId::TyreSets), args(header.packet_format))]
    pub tyre_sets: Option<F1PacketTyreSets>,
    /// Extended player car only motion data.
    /// Available from the 2023 format onwards.
    #[br(if(header.packet_id == PacketId::MotionEx), args(header.packet_format))]
    pub motion_ex: Option<F1PacketMotionEx>,
    /// Extra information that's only relevant to time trial game mode.
    /// Available from the 2024 format onwards.
    #[br(if(header.packet_id == PacketId::TimeTrial), args(header.packet_format))]
    pub time_trial: Option<F1PacketTimeTrial>,
}

/// F1 game packet's header. It contains metadata about the game,
/// the ongoing session, the frame this packet was sent on, and player car indexes.
#[non_exhaustive]
#[derive(BinRead, PartialEq, PartialOrd, Clone, Debug, Serialize, Deserialize)]
#[br(little)]
pub struct F1PacketHeader {
    /// Value of the "UDP Format" option in the game's telemetry settings.
    /// This crate currently supports formats in range `(2022..=2024)`.
    #[br(
        assert(
            (2022..=2024).contains(&packet_format),
            "Invalid or unsupported packet format: {}",
            packet_format
        )
    )]
    pub packet_format: u16,
    /// Game year (last two digits).
    /// Available from the 2023 format onwards.
    #[br(if(packet_format >= 2023))]
    pub game_year: u8,
    /// Game's major version - "X.00".
    pub game_major_version: u8,
    /// Game's minor version - "1.XX".
    pub game_minor_version: u8,
    /// Version of this packet type, all start from 1.
    pub packet_version: u8,
    /// Unique identifier for the packet type.
    pub packet_id: PacketId,
    /// Unique identifier for the session.
    pub session_uid: u64,
    /// Session timestamp.
    pub session_time: f32,
    /// Identifier for the frame the data was retrieved on.
    /// Goes back after a flashback is triggered.
    pub frame_identifier: u32,
    /// Overall identifier for the frame the data was retrieved on
    /// (i.e. it doesn't go back after flashbacks).
    /// Available from the 2023 format onwards.
    #[br(if(packet_format >= 2023))]
    pub overall_frame_identifier: u32,
    /// Index of player 1's car.
    #[br(
        map(u8_to_usize),
        assert(
            player_car_index < MAX_NUM_CARS,
            "Header has an invalid player 1 car index: {}",
            player_car_index
        )
    )]
    pub player_car_index: usize,
    /// Index of player 2's car in splitscreen mode.
    /// Set to 255 if not in splitscreen mode.
    #[br(map(u8_to_usize))]
    pub secondary_player_car_index: usize,
}
