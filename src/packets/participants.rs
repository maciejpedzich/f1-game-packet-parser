use super::{read_name, u8_to_bool};
use crate::constants::{Nationality, Platform, YourTelemetry};

use binrw::BinRead;
use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(
    BinRead, Eq, PartialEq, Ord, PartialOrd, Clone, Debug, Serialize, Deserialize,
)]
#[br(little, import(packet_format: u16))]
pub struct ParticipantsData {
    /// Whether the vehicle is controlled by AI.
    #[br(try_map(u8_to_bool))]
    pub ai_controlled: bool,
    /// Driver's ID.
    /// See [`driver_id`](mod@crate::constants::driver_id)
    /// for possible values.
    pub driver_id: u8,
    /// Unique ID for network players.
    pub network_id: u8,
    /// Team's ID.
    /// See [`team_id`](mod@crate::constants::team_id) for possible values.
    pub team_id: u8,
    /// Whether my team is being used.
    #[br(try_map(u8_to_bool))]
    pub my_team: bool,
    /// Race number of the car.
    pub race_number: u8,
    /// Driver's nationality.
    pub nationality: Nationality,
    /// Driver's name.
    #[br(try_map(read_name))]
    pub name: String,
    /// Player's UDP visibility setting.
    pub your_telemetry: Option<YourTelemetry>,
    /// Whether this player's "show online names" setting is on.
    /// Available from the 2023 format onwards.
    #[br(if(packet_format >= 2023), try_map(u8_to_bool))]
    pub show_online_names: bool,
    /// F1 World tech level.
    /// Available from the 2024 format onwards.
    #[br(if(packet_format >= 2024))]
    pub tech_level: u16,
    /// Player's platform.
    /// Available from the 2023 format onwards.
    #[br(if(packet_format >= 2023))]
    pub platform: Option<Platform>,
}
