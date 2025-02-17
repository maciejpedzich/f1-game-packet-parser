use super::u8_to_bool;
use crate::constants::{DriverStatus, PitStatus, ResultStatus};

use binrw::BinRead;
use serde::{Deserialize, Serialize};

/// Lap data for a car on track.
#[derive(
    BinRead, PartialEq, PartialOrd, Copy, Clone, Debug, Serialize, Deserialize,
)]
#[br(
    little,
    import(_packet_format: u16),
    assert(
        sector <= 2,
        "Lap data entry has an invalid sector number: {}",
        sector
    )
)]
pub struct LapData {
    /// Last lap time in milliseconds.
    pub last_lap_time_ms: u32,
    /// Current lap time in milliseconds.
    pub current_lap_time_ms: u32,
    /// Current sector 1 time in milliseconds.
    pub sector1_time_ms: u16,
    /// Current sector 2 time in milliseconds.
    pub sector2_time_ms: u16,
    /// The distance the vehicle is around current lap in metres.
    /// It may be negative if the start/finish line hasn’t been crossed yet.
    pub lap_distance: f32,
    /// The total distance the vehicle has gone around in this session in metres.
    /// It may be negative if the start/finish line hasn’t been crossed yet.
    pub total_distance: f32,
    /// Delta for the safety car in seconds.
    pub safety_car_delta: f32,
    /// Car's race position.
    pub car_position: u8,
    /// Current lap number.
    pub current_lap_num: u8,
    /// Car's pit status.
    pub pit_status: PitStatus,
    /// Number of pit stops taken in this race.
    pub num_pit_stops: u8,
    /// Zero-based number of the sector the driver is currently going through.
    /// S1 = 0, S2 = 1, S3 = 2.
    pub sector: u8,
    /// Whether the current lap is invalid.
    #[br(map(u8_to_bool))]
    pub current_lap_invalid: bool,
    /// Accumulated time penalties to be added in seconds.
    pub penalties: u8,
    /// Accumulated number of warnings issued.
    pub warnings: u8,
    /// Number of unserved drive through penalties left to serve.
    pub num_unserved_drive_through_pens: u8,
    /// Number of unserved stop-go penalties left to serve.
    pub num_unserved_stop_go_pens: u8,
    /// The grid position the vehicle started the race in.
    pub grid_position: u8,
    /// Status of the driver.
    pub driver_status: DriverStatus,
    /// Status of the driver's result.
    pub result_status: ResultStatus,
    /// Whether the pit lane timer is active.
    #[br(map(u8_to_bool))]
    pub pit_lane_timer_active: bool,
    /// Current time spent in the pit lane in milliseconds.
    pub pit_lane_time_in_lane_ms: u16,
    /// Time of the actual pit stop in milliseconds.
    pub pit_stop_timer_ms: u16,
    /// Whether the car should serve a penalty at this stop.
    #[br(map(u8_to_bool))]
    pub pit_stop_should_serve_pen: bool,
}
