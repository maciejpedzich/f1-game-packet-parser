pub mod constants;
pub mod packets;

use crate::constants::PacketId;
use crate::packets::{
    u8_to_usize, F1PacketCarDamage, F1PacketCarSetups, F1PacketCarStatus,
    F1PacketCarTelemetry, F1PacketEvent, F1PacketFinalClassification, F1PacketLap,
    F1PacketLobbyInfo, F1PacketMotion, F1PacketMotionEx, F1PacketParticipants,
    F1PacketSession, F1PacketSessionHistory, F1PacketTimeTrial, F1PacketTyreSets,
};

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
#[derive(BinRead, PartialEq, PartialOrd, Clone, Debug, Serialize, Deserialize)]
#[br(little)]
pub struct F1PacketHeader {
    /// Value of the "UDP Format" option in the game's telemetry settings.
    #[br(
        assert(
            (2022..=2024).contains(&packet_format),
            "Invalid or unsupported packet format: {}",
            packet_format
        )
    )]
    pub packet_format: u16,
    /// Game year (last two digits)
    /// Available from the 2023 format onwards.
    #[br(if(packet_format >= 2023))]
    pub game_year: u8,
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
    /// Overall identifier for the frame the data was retrieved on
    /// (doesn't go back after flashbacks).
    /// Available from the 2023 format onwards.
    #[br(if(packet_format >= 2023))]
    pub overall_frame_identifier: u32,
    /// Index of player's car in the array.
    #[br(map(u8_to_usize))]
    pub player_car_index: usize,
    /// Index of secondary player's car in the array in splitscreen mode.
    /// Set to 255 if not in splitscreen mode.
    #[br(map(u8_to_usize))]
    pub secondary_player_car_index: usize,
}

/// F1 game packet's body.
#[non_exhaustive]
#[derive(BinRead, PartialEq, PartialOrd, Clone, Debug, Serialize, Deserialize)]
#[br(little, import(packet_format: u16, packet_id: PacketId))]
pub struct F1PacketBody {
    /// Physics data for all cars in the session.
    #[br(if(packet_id == PacketId::Motion), args(packet_format))]
    pub motion: Option<F1PacketMotion>,
    /// Data about the ongoing session.
    #[br(if(packet_id == PacketId::Session), args(packet_format))]
    pub session: Option<F1PacketSession>,
    /// Lap data for all cars on track.
    #[br(if(packet_id == PacketId::LapData), args(packet_format))]
    pub lap: Option<F1PacketLap>,
    /// Details of events that happen during the course of a session.
    #[br(if(packet_id == PacketId::Event), args(packet_format))]
    pub event: Option<F1PacketEvent>,
    /// List of participants in the race.
    #[br(if(packet_id == PacketId::Participants), args(packet_format))]
    pub participants: Option<F1PacketParticipants>,
    /// Car setups for all cars in the session.
    #[br(if(packet_id == PacketId::CarSetups), args(packet_format))]
    pub car_setups: Option<F1PacketCarSetups>,
    /// Telemetry data for all cars in the session.
    #[br(if(packet_id == PacketId::CarTelemetry), args(packet_format))]
    pub car_telemetry: Option<F1PacketCarTelemetry>,
    /// Status data for all cars in the session.
    #[br(if(packet_id == PacketId::CarStatus), args(packet_format))]
    pub car_status: Option<F1PacketCarStatus>,
    /// Final classification confirmation at the end of a race.
    #[br(if(packet_id == PacketId::FinalClassification), args(packet_format))]
    pub final_classification: Option<F1PacketFinalClassification>,
    /// Details of players in a multiplayer lobby.
    #[br(if(packet_id == PacketId::LobbyInfo), args(packet_format))]
    pub lobby_info: Option<F1PacketLobbyInfo>,
    /// Car damage parameters for all cars in the session.
    #[br(if(packet_id == PacketId::CarDamage), args(packet_format))]
    pub car_damage: Option<F1PacketCarDamage>,
    /// Session history data for a specific car.
    #[br(if(packet_id == PacketId::SessionHistory), args(packet_format))]
    pub session_history: Option<F1PacketSessionHistory>,
    /// In-depth details about tyre sets assigned to a vehicle during the session.
    /// Available from the 2023 format onwards.
    #[br(if(packet_id == PacketId::TyreSets), args(packet_format))]
    pub tyre_sets: Option<F1PacketTyreSets>,
    /// Extended player car only motion data.
    /// Available from the 2023 format onwards.
    #[br(if(packet_id == PacketId::MotionEx), args(packet_format))]
    pub motion_ex: Option<F1PacketMotionEx>,
    /// Extra information that's only relevant to time trial game mode.
    /// Available from the 2024 format onwards.
    #[br(if(packet_id == PacketId::TimeTrial), args(packet_format))]
    pub time_trial: Option<F1PacketTimeTrial>,
}
