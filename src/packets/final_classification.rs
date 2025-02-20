use super::u8_to_usize;
use crate::constants::{ActualTyreCompound, ResultStatus, VisualTyreCompound};

use binrw::BinRead;
use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(BinRead, PartialEq, PartialOrd, Copy, Clone, Debug, Serialize, Deserialize)]
#[br(
    little,
    import(_packet_format: u16),
    assert(
        num_tyre_stints <= 8,
        "Final classification entry has an invalid number of tyre stints: {}",
        num_tyre_stints
    )
)]
pub struct FinalClassificationData {
    /// Finishing position.
    pub position: u8,
    /// Number of laps completed.
    pub num_laps: u8,
    /// Grid position of the car.
    pub grid_position: u8,
    /// Number of points scored.
    pub points: u8,
    /// Number of pit stops made.
    pub num_pit_stops: u8,
    /// Result status.
    pub result_status: ResultStatus,
    /// Best lap time of the session in milliseconds.
    pub best_lap_time_ms: u32,
    /// Total race time in seconds (without penalties).
    pub total_race_time: f64,
    /// Total penalties accumulated in seconds.
    pub penalties_time: u8,
    /// Number of penalties applied to this driver.
    pub num_penalties: u8,
    /// Number of tyre stints (no greater than 8).
    #[br(map(u8_to_usize))]
    pub num_tyre_stints: usize,
    /// Actual tyres used by the driver.
    pub tyre_stints_actual: [ActualTyreCompound; 8],
    /// Visual tyres used by the driver.
    pub tyre_stints_visual: [VisualTyreCompound; 8],
    /// The lap numbers the stints end on.
    pub tyre_stints_end_laps: [u8; 8],
}
