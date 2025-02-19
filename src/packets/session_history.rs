use super::u8_to_usize;
use crate::constants::{ActualTyreCompound, LapValid, VisualTyreCompound};

use binrw::BinRead;
use serde::{Deserialize, Serialize};

#[derive(
    BinRead, PartialEq, PartialOrd, Clone, Debug, Serialize, Deserialize,
)]
#[br(little, import(_packet_format: u16))]
pub struct LapHistoryData {
    /// Lap time in milliseconds.
    pub lap_time_ms: u32,
    /// Sector 1 time in milliseconds.
    pub sector1_time_ms: u16,
    /// Sector 2 time in milliseconds.
    pub sector2_time_ms: u16,
    /// Sector 3 time in milliseconds.
    pub sector3_time_ms: u16,
    /// Bitmap of lap validity across all sectors and overall.
    #[br(map(LapValid::from_bits_truncate))]
    pub lap_valid_bit_flags: LapValid,
}

#[derive(
    BinRead, PartialEq, PartialOrd, Clone, Debug, Serialize, Deserialize,
)]
#[br(little, import(_packet_format: u16))]
pub struct TyreStintHistoryData {
    /// Lap the tyre usage ends on (255 if current tyre)
    #[br(map(u8_to_usize))]
    pub end_lap: usize,
    /// Actual tyre compound used
    pub actual_tyre_compound: ActualTyreCompound,
    /// Visual tyre compound used
    pub visual_tyre_compound: VisualTyreCompound,
}
