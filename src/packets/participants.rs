use super::{read_name, u8_to_bool};
use crate::constants::{Nationality, TeamId, YourTelemetry};

use binrw::BinRead;
use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(
    BinRead, PartialEq, PartialOrd, Clone, Debug, Serialize, Deserialize,
)]
#[br(little, import(_packet_format: u16))]
pub struct ParticipantsData {
    /// Whether the vehicle is controlled by AI
    #[br(map(u8_to_bool))]
    pub ai_controlled: bool,
    /// Driver's ID
    pub driver_id: u8,
    /// Unique ID for network players
    pub network_id: u8,
    /// Team's ID
    pub team_id: TeamId,
    /// Whether my team is being used
    #[br(map(u8_to_bool))]
    pub my_team: bool,
    /// Race number of the car
    pub race_number: u8,
    /// Driver's nationality
    pub nationality: Nationality,
    /// Driver's name
    #[br(try_map(read_name))]
    pub name: String,
    /// Player's UDP visibility setting
    pub your_telemetry: Option<YourTelemetry>,
}
