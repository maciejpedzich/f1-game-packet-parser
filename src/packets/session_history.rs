use super::u8_to_usize;
use crate::constants::{ActualTyreCompound, LapValid, VisualTyreCompound};

use binrw::BinRead;
use serde::{Deserialize, Serialize};

pub(super) const MAX_NUM_LAPS: usize = 100;
pub(super) const MAX_NUM_TYRE_STINTS: usize = 8;

#[non_exhaustive]
#[derive(
    BinRead, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Debug, Serialize, Deserialize,
)]
#[br(little, import(packet_format: u16))]
pub struct LapHistoryData {
    /// Lap time in milliseconds.
    pub lap_time_ms: u32,
    /// Sector 1 time milliseconds part.
    pub sector1_time_ms_part: u16,
    /// Sector 1 whole minute part.
    /// Available from the 2023 format onwards.
    #[br(if(packet_format >= 2023))]
    pub sector1_time_minutes_part: u8,
    /// Sector 2 time milliseconds part.
    pub sector2_time_ms_part: u16,
    /// Sector 2 whole minute part.
    /// Available from the 2023 format onwards.
    #[br(if(packet_format >= 2023))]
    pub sector2_time_minutes_part: u8,
    /// Sector 3 time milliseconds part.
    pub sector3_time_ms_part: u16,
    /// Sector 3 whole minute part.
    /// Available from the 2023 format onwards.
    #[br(if(packet_format >= 2023))]
    pub sector3_time_minutes: u8,
    /// Bitmap of lap validity across all sectors and overall.
    #[br(map(LapValid::from_bits_retain))]
    pub lap_valid_bit_flags: LapValid,
}

#[non_exhaustive]
#[derive(
    BinRead, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Debug, Serialize, Deserialize,
)]
#[br(little, import(_packet_format: u16))]
pub struct TyreStintHistoryData {
    /// Lap the tyre usage ends on (255 if current tyre).
    #[br(map(u8_to_usize))]
    pub end_lap: usize,
    /// Actual tyre compound used.
    pub actual_tyre_compound: ActualTyreCompound,
    /// Visual tyre compound used.
    pub visual_tyre_compound: VisualTyreCompound,
}

pub(super) fn get_lap_history_raw_size(packet_format: u16) -> usize {
    if packet_format >= 2023 {
        14
    } else {
        11
    }
}
