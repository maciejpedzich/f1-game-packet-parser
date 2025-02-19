use super::{read_name, u8_to_bool};
use crate::constants::{Nationality, ReadyStatus, TeamId};
use binrw::BinRead;
use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(
    BinRead, PartialEq, PartialOrd, Clone, Debug, Serialize, Deserialize,
)]
#[br(little, import(_packet_format: u16))]
pub struct LobbyInfoData {
    /// Whether the vehicle is controlled by AI.
    #[br(map(u8_to_bool))]
    pub ai_controlled: bool,
    /// Team's ID.
    pub team_id: TeamId,
    /// Driver's nationality.
    pub nationality: Nationality,
    /// Driver's name.
    #[br(try_map(read_name))]
    pub name: String,
    /// Player's car number
    pub car_number: u8,
    /// Readiness status.
    pub ready_status: ReadyStatus,
}
