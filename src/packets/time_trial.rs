use super::{u8_to_bool, u8_to_usize};
use crate::constants::{GearboxAssist, TractionControl};

use binrw::BinRead;
use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(
    BinRead, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Debug, Serialize, Deserialize,
)]
#[br(little, import(_packet_format: u16))]
pub struct TimeTrialDataSet {
    /// Index of the car this data set relates to.
    #[br(map(u8_to_usize))]
    pub vehicle_index: usize,
    /// Team's ID.
    pub team_id: u8,
    /// Lap time in milliseconds.
    pub lap_time_ms: u32,
    /// Sector 1 time in milliseconds.
    pub sector1_time_ms: u32,
    /// Sector 2 time in milliseconds.
    pub sector2_time_ms: u32,
    /// Sector 3 time in milliseconds.
    pub sector3_time_ms: u32,
    /// Type of traction control assist enabled.
    pub traction_control: TractionControl,
    /// Type of gearbox assist enabled.
    pub gearbox_assist: GearboxAssist,
    /// Whether ABS is enabled.
    #[br(try_map(u8_to_bool))]
    pub anti_lock_brakes: bool,
    /// Whether equal car performance is enabled.
    #[br(try_map(u8_to_bool))]
    pub equal_car_performance: bool,
    /// Whether custom setup is in use.
    #[br(try_map(u8_to_bool))]
    pub custom_setup: bool,
    /// Whether this lap is valid.
    #[br(try_map(u8_to_bool))]
    pub valid: bool,
}
