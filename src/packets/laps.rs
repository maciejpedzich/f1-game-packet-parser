use super::u8_to_bool;
use crate::constants::{DriverStatus, PitStatus, ResultStatus, Sector};

use binrw::BinRead;
use serde::{Deserialize, Serialize};

/// Lap data for a car on track.
#[non_exhaustive]
#[derive(BinRead, PartialEq, PartialOrd, Copy, Clone, Debug, Serialize, Deserialize)]
#[br(little, import(packet_format: u16))]
pub struct LapData {
    /// Last lap time in milliseconds.
    pub last_lap_time_ms: u32,
    /// Current lap time in milliseconds.
    pub current_lap_time_ms: u32,
    /// Current sector 1 time millisecond part.
    pub sector1_time_ms_part: u16,
    /// Sector 1 whole minute part.
    /// Available from the 2023 format onwards.
    #[br(if(packet_format >= 2023))]
    pub sector1_time_minutes_part: u8,
    /// Current sector 2 time millisecond part.
    pub sector2_time_ms_part: u16,
    /// Sector 2 whole minute part.
    /// Available from the 2023 format onwards.
    #[br(if(packet_format >= 2023))]
    pub sector2_time_minutes_part: u8,
    /// Time delta to car in front in milliseconds.
    /// Available from the 2023 format onwards.
    #[br(if(packet_format >= 2023))]
    pub delta_to_car_in_front_ms_part: u16,
    /// Time delta to car in front whole minute part.
    /// Available from the 2024 format onwards.
    #[br(if(packet_format >= 2024))]
    pub delta_to_car_in_front_minutes_part: u8,
    /// Time delta to race leader in milliseconds.
    /// Available from the 2023 format onwards.
    #[br(if(packet_format >= 2023))]
    pub delta_to_race_leader_ms: u16,
    /// Time delta to car in front whole minute part.
    /// Available from the 2024 format onwards.
    #[br(if(packet_format >= 2024))]
    pub delta_to_race_leader_minutes_part: u8,
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
    pub sector: Sector,
    /// Whether the current lap is invalid.
    #[br(try_map(u8_to_bool))]
    pub current_lap_invalid: bool,
    /// Accumulated time penalties to be added in seconds.
    pub penalties: u8,
    /// Accumulated number of warnings issued.
    pub total_warnings: u8,
    /// Accumulated number of corner cutting warnings issued.
    /// Available from the 2023 format onwards.
    #[br(if(packet_format >= 2023))]
    pub corner_cutting_warnings: u8,
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
    #[br(try_map(u8_to_bool))]
    pub pit_lane_timer_active: bool,
    /// Current time spent in the pit lane in milliseconds.
    pub pit_lane_time_in_lane_ms: u16,
    /// Time of the actual pit stop in milliseconds.
    pub pit_stop_timer_ms: u16,
    /// Whether the car should serve a penalty at this stop.
    #[br(try_map(u8_to_bool))]
    pub pit_stop_should_serve_pen: bool,
    /// Fastest speed through speed trap for this car in kilometres per hour.
    /// Available from the 2024 format onwards.
    #[br(if(packet_format >= 2024))]
    pub speed_trap_fastest_speed: f32,
    /// Number of the lap the fastest speed was achieved on
    /// (255 means "not set").
    /// Available from the 2024 format onwards.
    #[br(if(packet_format >= 2024))]
    pub speed_trap_fastest_lap: u8,
}
