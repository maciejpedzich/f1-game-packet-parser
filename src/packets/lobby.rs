use super::{read_name, u8_to_bool};
use crate::constants::{Nationality, Platform, ReadyStatus};

use binrw::BinRead;
use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(
    BinRead, PartialEq, PartialOrd, Clone, Debug, Serialize, Deserialize,
)]
#[br(little, import(packet_format: u16))]
pub struct LobbyInfoData {
    /// Whether the vehicle is controlled by AI.
    #[br(map(u8_to_bool))]
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
    /// Readiness status.
    pub ready_status: ReadyStatus,
}
