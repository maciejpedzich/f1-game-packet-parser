pub mod motion;
pub mod session;
pub mod lap_data;

use crate::packets::motion::CarMotionData;

use crate::constants::{
    BrakingAssist, DynamicRacingLine,
    DynamicRacingLineType, ForecastAccuracy, Formula, GameMode,
    GearboxAssist, Ruleset, SafetyCarStatus, SessionLength, SessionType,
    TrackId, Weather, MAX_AI_DIFFICULTY, MAX_NUM_CARS, MAX_NUM_MARSHAL_ZONES,
    MAX_NUM_WEATHER_FORECAST_SAMPLES,
};
use crate::packets::session::{MarshalZone, WeatherForecastSample};

use crate::packets::lap_data::LapData;
use binrw::BinRead;
use serde::{Deserialize, Serialize};
use std::string::FromUtf8Error;

/// Physics data for all the cars being driven.
#[non_exhaustive]
#[derive(
    BinRead, PartialEq, PartialOrd, Clone, Debug, Serialize, Deserialize,
)]
#[br(little, import(packet_format: u16))]
pub struct F1PacketMotionData {
    /// Motion data for all cars on track.
    #[br(count(MAX_NUM_CARS), args{ inner: (packet_format,) })]
    pub car_motion_data: Vec<CarMotionData>,
    /// Extra player car only motion data (2022 format only).
    #[br(if(packet_format == 2022))]
    pub motion_ex_data: Option<F1PacketMotionExData>,
}

/// Data about the ongoing session.
#[allow(clippy::struct_excessive_bools)]
#[non_exhaustive]
#[derive(
    BinRead, PartialEq, PartialOrd, Clone, Debug, Serialize, Deserialize,
)]
#[br(
    little,
    import(packet_format: u16),
    assert(
        num_marshal_zones <= MAX_NUM_MARSHAL_ZONES,
        "Session packet has an invalid number of marshal zones: {}",
        num_marshal_zones
    ),
    assert(
        num_weather_forecast_samples <= MAX_NUM_WEATHER_FORECAST_SAMPLES,
        "Session packet has an invalid number of weather forecast samples: {}",
        num_weather_forecast_samples
    ),
    assert(
        ai_difficulty <= MAX_AI_DIFFICULTY,
        "Session packet has an invalid AI difficulty value: {}",
        ai_difficulty
    ),
)]
pub struct F1PacketSessionData {
    /// Current weather.
    pub weather: Weather,
    /// Track temperature in degrees Celsius.
    pub track_temperature: i8,
    /// Air temperature in degrees Celsius.
    pub air_temperature: i8,
    /// Total number of laps in this session.
    pub total_laps: u8,
    /// Track's length in metres.
    pub track_length: u16,
    /// Session's type.
    pub session_type: SessionType,
    /// Unique identifier of the track.
    pub track_id: TrackId,
    /// Formula of cars being raced.
    pub formula: Formula,
    /// Time left in the session in seconds.
    pub session_time_left: u16,
    /// Session's duration in seconds.
    pub session_duration: u16,
    /// Pit lane's speed limit in kilometres per hour.
    pub pit_speed_limit: u8,
    /// Whether the game is paused.
    #[br(map(u8_to_bool))]
    pub game_paused: bool,
    /// Whether the player is spectating.
    #[br(map(u8_to_bool))]
    pub is_spectating: bool,
    /// Index of the car being spectated.
    #[br(map(u8_to_usize))]
    pub spectator_car_index: usize,
    /// Whether SLI Pro support is active.
    #[br(map(u8_to_bool))]
    pub sli_pro_native_support: bool,
    /// Number of marshal zones to follow (no greater than 21).
    #[br(map(u8_to_usize))]
    pub num_marshal_zones: usize,
    /// List of marshal zones.
    /// Has a size of 21 regardless of the `num_marshal_zones` value.
    #[br(count(MAX_NUM_MARSHAL_ZONES), args{ inner: (packet_format,) })]
    pub marshal_zones: Vec<MarshalZone>,
    /// Safety car deployment status.
    pub safety_car_status: SafetyCarStatus,
    /// Whether this game is online.
    #[br(map(u8_to_bool))]
    pub network_game: bool,
    /// Number of weather samples to follow (no greater than 56).
    #[br(map(u8_to_usize))]
    pub num_weather_forecast_samples: usize,
    /// List of up to weather forecast samples.
    /// Has a size of 56 regardless of the `num_weather_forecast_samples` value.
    #[br(
        count(MAX_NUM_WEATHER_FORECAST_SAMPLES),
        args{ inner: (packet_format,) }
    )]
    pub weather_forecast_samples: Vec<WeatherForecastSample>,
    /// Weather forecast accuracy.
    pub forecast_accuracy: ForecastAccuracy,
    /// AI difficulty rating in range `(0..=110)`.
    pub ai_difficulty: u8,
    /// Identifier for season - persists across saves.
    pub season_link_identifier: u32,
    /// Identifier for weekend - persists across saves.
    pub weekend_link_identifier: u32,
    /// Identifier for session - persists across saves.
    pub session_link_identifier: u32,
    /// Ideal lap for the player to pit on for current strategy.
    pub pit_stop_window_ideal_lap: u8,
    /// The latest lap for the player to pit on for current strategy.
    pub pit_stop_window_latest_lap: u8,
    /// Predicted position for the player to rejoin at.
    pub pit_stop_rejoin_position: u8,
    /// Whether the steering assist is enabled.
    #[br(map(u8_to_bool))]
    pub steering_assist: bool,
    /// Type of braking assist enabled.
    pub braking_assist: BrakingAssist,
    /// Type of gearbox assist enabled.
    pub gearbox_assist: GearboxAssist,
    /// Whether the pit assist is enabled.
    #[br(map(u8_to_bool))]
    pub pit_assist: bool,
    /// Whether the pit release assist is enabled.
    #[br(map(u8_to_bool))]
    pub pit_release_assist: bool,
    /// Whether the ERS assist is enabled.
    #[br(map(u8_to_bool))]
    pub ers_assist: bool,
    /// Whether the DRS assist is enabled.
    #[br(map(u8_to_bool))]
    pub drs_assist: bool,
    /// Type of the dynamic racing line assist.
    pub dynamic_racing_line: DynamicRacingLine,
    /// Type of the dynamic racing line (2D/3D).
    pub dynamic_racing_line_type: DynamicRacingLineType,
    /// Game mode's identifier.
    pub game_mode: GameMode,
    /// Ruleset's identifier.
    pub ruleset: Ruleset,
    /// Session's length.
    pub session_length: SessionLength,
}

/// Data about all the lap times of cars in the session.
#[non_exhaustive]
#[derive(
    BinRead, PartialEq, PartialOrd, Clone, Debug, Serialize, Deserialize,
)]
#[br(little, import(packet_format: u16))]
pub struct F1PacketLapData {
    /// Lap data for all cars on track.
    #[br(count(MAX_NUM_CARS), args{ inner: (packet_format,) })]
    pub lap_data: Vec<LapData>,
    /// Index of personal best car in time trial mode (255 if invalid).
    #[br(map(u8_to_usize))]
    pub time_trial_pb_car_index: usize,
    /// Index of rival's car in time trial mode (255 if invalid).
    #[br(map(u8_to_usize))]
    pub time_trial_rival_car_index: usize,
}

/// Extended motion data for player's car. Available as a:
/// - part of [`F1PacketMotionData`] in the 2022 format
/// - standalone packet from the 2023 format onwards
#[non_exhaustive]
#[derive(
    BinRead, PartialEq, PartialOrd, Copy, Clone, Debug, Serialize, Deserialize,
)]
#[br(little, import(_packet_format: u16))]
pub struct F1PacketMotionExData {
    /// Positions of suspension for each wheel.
    /// See [`wheel_index`](mod@crate::constants::wheel_index)
    /// for wheel order.
    pub suspension_position: [f32; 4],
    /// Velocities of suspension.
    /// See [`wheel_index`](mod@crate::constants::wheel_index)
    /// for wheel order.
    pub suspension_velocity: [f32; 4],
    /// Accelerations of suspension in the following order:
    /// See [`wheel_index`](mod@crate::constants::wheel_index)
    /// for wheel order.
    pub suspension_acceleration: [f32; 4],
    /// Speed of each wheel in the following order:
    /// See [`wheel_index`](mod@crate::constants::wheel_index)
    /// for wheel order.
    pub wheel_speed: [f32; 4],
    /// Slip ratio of each wheel in the following order:
    /// See [`wheel_index`](mod@crate::constants::wheel_index)
    /// for wheel order.
    pub wheel_slip: [f32; 4],
    /// X velocity in local space.
    pub local_velocity_x: f32,
    /// Y velocity in local space.
    pub local_velocity_y: f32,
    /// Z velocity in local space.
    pub local_velocity_z: f32,
    /// Angular velocity X component.
    pub angular_velocity_x: f32,
    /// Angular velocity Y component.
    pub angular_velocity_y: f32,
    /// Angular velocity Z component.
    pub angular_velocity_z: f32,
    /// Angular acceleration X component.
    pub angular_acceleration_x: f32,
    /// Angular acceleration Y component.
    pub angular_acceleration_y: f32,
    /// Angular acceleration Z component.
    pub angular_acceleration_z: f32,
    /// Current front wheels angle in radians.
    pub front_wheels_angle: f32,
}

pub(crate) fn u8_to_bool(value: u8) -> bool {
    value == 1
}

pub(crate) fn u8_to_usize(value: u8) -> usize {
    value as usize
}

pub(crate) fn read_name(bytes: [u8; 48]) -> Result<String, FromUtf8Error> {
    let first_nul_index =
        bytes.iter().position(|&byte| byte == b'\0').unwrap_or(bytes.len());

    String::from_utf8(bytes[..first_nul_index].to_vec())
}
