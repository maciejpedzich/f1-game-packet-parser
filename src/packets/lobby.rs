use super::{read_name, u8_to_bool};
use crate::constants::{Nationality, Platform, ReadyStatus, YourTelemetry};

use binrw::BinRead;
use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(
    BinRead, Eq, PartialEq, Ord, PartialOrd, Clone, Debug, Serialize, Deserialize,
)]
#[br(little, import(packet_format: u16))]
pub struct LobbyInfoData {
    /// Whether the vehicle is controlled by AI.
    #[br(try_map(u8_to_bool))]
    pub ai_controlled: bool,
    /// Team's ID.
    pub team_id: u8,
    /// Driver's nationality.
    pub nationality: Nationality,
    /// Player's platform.
    /// Available from the 2023 format onwards.
    #[br(if(packet_format >= 2023))]
    pub platform: Option<Platform>,
    /// Driver's name.
    #[br(try_map(read_name))]
    pub name: String,
    /// Player's car number.
    pub car_number: u8,
    /// The player's "Your Telemetry" visibility setting.
    /// Available from the 2024 format onwards.
    #[br(if(packet_format >= 2024))]
    pub your_telemetry: Option<YourTelemetry>,
    /// Whether the player has enabled the "Show online names" setting.
    /// Available from the 2024 format onwards.
    #[br(if(packet_format >= 2024), try_map(u8_to_bool))]
    pub show_online_names: bool,
    /// F1 World tech level.
    /// Available from the 2024 format onwards.
    #[br(if(packet_format >= 2024))]
    pub tech_level: u16,
    /// Readiness status.
    pub ready_status: ReadyStatus,
}
