pub mod constants;
pub mod packets;

use crate::constants::PacketId;
use crate::packets::{F1PacketMotionData, F1PacketSessionData};

use binrw::io::Cursor;
use binrw::{BinRead, BinReaderExt, BinResult};
use serde::{Deserialize, Serialize};

/// Attempts to extract game packet data from a slice of bytes.
/// See [`binrw::Error`] for possible error variants.
pub fn parse(data: &[u8]) -> BinResult<F1Packet> {
    let mut cursor = Cursor::new(data);
    let header: F1PacketHeader = cursor.read_le()?;
    let body: F1PacketBody =
        cursor.read_le_args((header.packet_format, header.packet_id))?;

    Ok(F1Packet { header, body })
}

#[derive(PartialEq, PartialOrd, Clone, Debug, Serialize, Deserialize)]
pub struct F1Packet {
    pub header: F1PacketHeader,
    pub body: F1PacketBody,
}

/// F1 game packet's header.
#[non_exhaustive]
#[derive(
    BinRead, PartialEq, PartialOrd, Clone, Debug, Serialize, Deserialize,
)]
#[br(
    little,
    assert(
        (2022..=2024).contains(&packet_format),
        "Invalid or unsupported packet format: {}",
        packet_format
    )
)]
pub struct F1PacketHeader {
    /// Value of the "UDP Format" option in the game's telemetry settings.
    pub packet_format: u16,
    /// Game's major version - "X.00".
    pub game_major_version: u8,
    /// Game's minor version - "1.XX".
    pub game_minor_version: u8,
    /// Version of this packet type, all start from 1.
    pub packet_version: u8,
    /// Identifier for the packet type.
    pub packet_id: PacketId,
    /// Unique identifier for the session.
    pub session_uid: u64,
    /// Session timestamp.
    pub session_time: f32,
    /// Identifier for the frame the data was retrieved on.
    pub frame_identifier: u32,
    /// Index of player's car in the array.
    pub player_car_index: u8,
    /// Index of secondary player's car in the array in splitscreen mode.
    /// Set to 255 if not in splitscreen mode.
    pub secondary_player_car_index: u8,
}

/// F1 game packet's body.
#[non_exhaustive]
#[derive(
    BinRead, PartialEq, PartialOrd, Clone, Debug, Serialize, Deserialize,
)]
#[br(little, import(packet_format: u16, packet_id: PacketId))]
pub struct F1PacketBody {
    /// Physics data for all the cars being driven.
    #[br(if(packet_id == PacketId::Motion), args(packet_format))]
    pub motion: Option<F1PacketMotionData>,
    /// Data about the ongoing session.
    #[br(if(packet_id == PacketId::Session), args(packet_format))]
    pub session: Option<F1PacketSessionData>,
}
