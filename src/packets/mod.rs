pub mod car_damage;
pub mod car_setups;
pub mod car_status;
pub mod car_telemetry;
pub mod event;
pub mod final_classification;
pub mod lap;
pub mod lobby;
pub mod motion;
pub mod participants;
pub mod session;
pub mod session_history;
pub mod tyre_sets;

use crate::constants::{
    BrakingAssist, DynamicRacingLine, DynamicRacingLineType, ForecastAccuracy,
    Formula, GameMode, GearboxAssist, MfdPanelIndex, RuleSet, SafetyCarStatus,
    SessionLength, SessionType, SpeedUnit, TemperatureUnit, TrackId, Weather,
    MAX_NUM_CARS,
};
use crate::packets::car_damage::CarDamageData;
use crate::packets::car_setups::CarSetupData;
use crate::packets::car_status::CarStatusData;
use crate::packets::car_telemetry::CarTelemetryData;
use crate::packets::event::EventDataDetails;
use crate::packets::final_classification::FinalClassificationData;
use crate::packets::lap::LapData;
use crate::packets::lobby::LobbyInfoData;
use crate::packets::motion::CarMotionData;
use crate::packets::participants::ParticipantsData;
use crate::packets::session::{MarshalZone, WeatherForecastSample};
use crate::packets::session_history::{LapHistoryData, TyreStintHistoryData};
use crate::packets::tyre_sets::TyreSetData;

use binrw::{BinRead, BinResult};
use serde::{Deserialize, Serialize};
use std::string::FromUtf8Error;

/// Physics data for all the cars being driven.
#[non_exhaustive]
#[derive(
    BinRead, PartialEq, PartialOrd, Clone, Debug, Serialize, Deserialize,
)]
#[br(little, import(packet_format: u16))]
pub struct F1PacketMotion {
    /// Motion data for all cars on track.
    #[br(count(MAX_NUM_CARS), args{ inner: (packet_format,) })]
    pub car_motion_data: Vec<CarMotionData>,
    /// Extra player car only motion data.
    /// Only in the 2022 format.
    #[br(if(packet_format == 2022))]
    pub motion_ex_data: Option<F1PacketMotionEx>,
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
        num_marshal_zones <= 21,
        "Session packet has an invalid number of marshal zones: {}",
        num_marshal_zones
    ),
    assert(
        num_weather_forecast_samples <= 56,
        "Session packet has an invalid number of weather forecast samples: {}",
        num_weather_forecast_samples
    ),
    assert(
        ai_difficulty <= 110,
        "Session packet has an invalid AI difficulty value: {}",
        ai_difficulty
    ),
)]
pub struct F1PacketSession {
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
    #[br(try_map(u8_to_bool))]
    pub game_paused: bool,
    /// Whether the player is spectating.
    #[br(try_map(u8_to_bool))]
    pub is_spectating: bool,
    /// Index of the car being spectated.
    #[br(map(u8_to_usize))]
    pub spectator_car_index: usize,
    /// Whether SLI Pro support is active.
    #[br(try_map(u8_to_bool))]
    pub sli_pro_native_support: bool,
    /// Number of marshal zones to follow (no greater than 21).
    #[br(map(u8_to_usize))]
    pub num_marshal_zones: usize,
    /// List of marshal zones.
    /// Has a size of 21 regardless of the
    /// [`num_marshal_zones`](field@F1PacketSession::num_marshal_zones) value.
    #[br(count(21), args{ inner: (packet_format,) })]
    pub marshal_zones: Vec<MarshalZone>,
    /// Safety car deployment status.
    pub safety_car_status: SafetyCarStatus,
    /// Whether this game is online.
    #[br(try_map(u8_to_bool))]
    pub network_game: bool,
    /// Number of weather samples to follow (no greater than 56).
    #[br(map(u8_to_usize))]
    pub num_weather_forecast_samples: usize,
    /// List of up to weather forecast samples.
    /// Has a size of 56 regardless of the
    /// [`num_weather_forecast_samples`](field@F1PacketSession::num_weather_forecast_samples)
    /// value.
    #[br(count(56), args{ inner: (packet_format,) })]
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
    #[br(try_map(u8_to_bool))]
    pub steering_assist: bool,
    /// Type of braking assist enabled.
    pub braking_assist: BrakingAssist,
    /// Type of gearbox assist enabled.
    pub gearbox_assist: GearboxAssist,
    /// Whether the pit assist is enabled.
    #[br(try_map(u8_to_bool))]
    pub pit_assist: bool,
    /// Whether the pit release assist is enabled.
    #[br(try_map(u8_to_bool))]
    pub pit_release_assist: bool,
    /// Whether the ERS assist is enabled.
    #[br(try_map(u8_to_bool))]
    pub ers_assist: bool,
    /// Whether the DRS assist is enabled.
    #[br(try_map(u8_to_bool))]
    pub drs_assist: bool,
    /// Type of the dynamic racing line assist.
    pub dynamic_racing_line: DynamicRacingLine,
    /// Type of the dynamic racing line (2D/3D).
    pub dynamic_racing_line_type: DynamicRacingLineType,
    /// Game mode's identifier.
    pub game_mode: GameMode,
    /// Rule set's identifier.
    pub rule_set: RuleSet,
    /// Local time of day as minutes since midnight.
    pub time_of_day: u32,
    /// Session's length.
    pub session_length: SessionLength,
    /// Speed unit used by player 1.
    /// Available from the 2023 format onwards.
    #[br(if(packet_format >= 2023))]
    pub speed_unit_lead_player: Option<SpeedUnit>,
    /// Temperature unit used by player 1.
    /// Available from the 2023 format onwards.
    #[br(if(packet_format >= 2023))]
    pub temperature_unit_lead_player: Option<TemperatureUnit>,
    /// Speed unit used by player 2.
    /// Available from the 2023 format onwards.
    #[br(if(packet_format >= 2023))]
    pub speed_unit_secondary_player: Option<SpeedUnit>,
    /// Temperature unit used by player 2.
    /// Available from the 2023 format onwards.
    #[br(if(packet_format >= 2023))]
    pub temperature_unit_secondary_player: Option<TemperatureUnit>,
    /// Number of full safety cars called during the session.
    /// Available from the 2023 format onwards.
    #[br(if(packet_format >= 2023))]
    pub num_safety_car_periods: u8,
    /// Number of virtual safety cars called during the session.
    /// Available from the 2023 format onwards.
    #[br(if(packet_format >= 2023))]
    pub num_virtual_safety_car_periods: u8,
    /// Number of red flags called during the session.
    /// Available from the 2023 format onwards.
    #[br(if(packet_format >= 2023))]
    pub num_red_flag_periods: u8,
    /// Whether equal car performance is enabled.
    /// Available from the 2024 format onwards.
    #[br(try_map(u8_to_bool))]
    pub equal_car_performance: bool,
}

/// Data about all the lap times of cars in the session.
#[non_exhaustive]
#[derive(
    BinRead, PartialEq, PartialOrd, Clone, Debug, Serialize, Deserialize,
)]
#[br(little, import(packet_format: u16))]
pub struct F1PacketLap {
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

/// Various notable events that happen during a session.
#[non_exhaustive]
#[derive(
    BinRead, PartialEq, PartialOrd, Clone, Debug, Serialize, Deserialize,
)]
#[br(little, import(_packet_format: u16))]
pub struct F1PacketEvent {
    /// 4-letter event code.
    #[br(
        try_map(|bytes: [u8; 4]| String::from_utf8(bytes.to_vec())),
        restore_position
    )]
    pub event_string_code: String,
    /// Extra data for this event.
    pub event_details: EventDataDetails,
}

/// Data of participants in the session, mostly relevant for multiplayer.
#[non_exhaustive]
#[derive(
    BinRead, PartialEq, PartialOrd, Clone, Debug, Serialize, Deserialize,
)]
#[br(
    little,
    import(packet_format: u16),
    assert(
        num_cars <= MAX_NUM_CARS,
        "Participants packet has an invalid number of cars: {}",
        num_cars
    )
)]
pub struct F1PacketParticipants {
    /// Number of active cars in the data (no greater than 22)
    #[br(map(u8_to_usize))]
    pub num_cars: usize,
    /// Data for all participants.
    /// Should have a size equal to
    /// [`num_cars`](field@F1PacketParticipants::num_cars).
    #[br(count(num_cars), args{ inner: (packet_format,) })]
    pub participants: Vec<ParticipantsData>,
}

/// Car setups for all cars in the race.
/// In multiplayer games, other player cars will appear as blank.
/// You will only be able to see your car setup and AI cars.
#[non_exhaustive]
#[derive(
    BinRead, PartialEq, PartialOrd, Clone, Debug, Serialize, Deserialize,
)]
#[br(little, import(packet_format: u16))]
pub struct F1PacketCarSetups {
    /// Setup data for all cars on track.
    /// Should have a size of 22.
    #[br(count(MAX_NUM_CARS), args{ inner: (packet_format,) })]
    pub car_setups: Vec<CarSetupData>,
}

/// Telemetry (such as speed, DRS, throttle application, etc.)
/// for all cars in the race.
#[derive(
    BinRead, PartialEq, PartialOrd, Clone, Debug, Serialize, Deserialize,
)]
#[br(
    little,
    import(packet_format: u16),
    assert(
        (-1..=8).contains(&suggested_gear),
        "Car telemetry entry has an invalid suggested gear value: {}",
        suggested_gear
    ),
)]
pub struct F1PacketCarTelemetry {
    /// Telemetry data for all cars on track.
    /// Should have a size of 22.
    #[br(count(MAX_NUM_CARS), args{ inner: (packet_format,) })]
    pub car_telemetry_data: Vec<CarTelemetryData>,
    /// Index of currently open MFD panel.
    pub mfd_panel_index: MfdPanelIndex,
    /// See [`mfd_panel_index`](field@F1PacketCarTelemetry::mfd_panel_index).
    pub mfd_panel_index_secondary_player: MfdPanelIndex,
    /// Suggested gear (0 if no gear suggested).
    pub suggested_gear: i8,
}

/// Car status data for each car in the race.
#[non_exhaustive]
#[derive(
    BinRead, PartialEq, PartialOrd, Clone, Debug, Serialize, Deserialize,
)]
#[br(little, import(packet_format: u16))]
pub struct F1PacketCarStatus {
    /// Status data for all cars. Should have a size of 22.
    #[br(count(MAX_NUM_CARS), args{ inner: (packet_format,) })]
    pub car_status_data: Vec<CarStatusData>,
}

/// Final classification confirmation at the end of a race.
#[non_exhaustive]
#[derive(
    BinRead, PartialEq, PartialOrd, Clone, Debug, Serialize, Deserialize,
)]
#[br(
    little,
    import(packet_format: u16),
    assert(
        num_cars <= MAX_NUM_CARS,
        "Final classification packet has an invalid number of cars: {}",
        num_cars
    )
)]
pub struct F1PacketFinalClassification {
    /// Number of cars in the final classification (no greater than 22).
    #[br(map(u8_to_usize))]
    pub num_cars: usize,
    /// Final classification data for all cars.
    /// Should have a size equal to
    /// [`num_cars`](field@F1PacketFinalClassification::num_cars).
    #[br(count(num_cars), args{ inner: (packet_format,) })]
    pub final_classification_data: Vec<FinalClassificationData>,
}

/// Packet detailing all the players that are currently in a multiplayer lobby.
#[non_exhaustive]
#[derive(
    BinRead, PartialEq, PartialOrd, Clone, Debug, Serialize, Deserialize,
)]
#[br(
    little,
    import(packet_format: u16),
    assert(
        num_players <= MAX_NUM_CARS,
        "Lobby packet has an invalid number of players: {}",
        num_players
    )
)]
pub struct F1PacketLobbyInfo {
    /// Number of players in the lobby (no greater than 22).
    #[br(map(u8_to_usize))]
    pub num_players: usize,
    /// Lobby info data for all players.
    /// Should have a size equal to
    /// [`num_players`](field@F1PacketLobbyInfo::num_players).
    #[br(count(num_players), args{ inner: (packet_format,) })]
    pub lobby_info_data: Vec<LobbyInfoData>,
}

/// Car damage parameters for all cars in the session.
#[non_exhaustive]
#[derive(
    BinRead, PartialEq, PartialOrd, Clone, Debug, Serialize, Deserialize,
)]
#[br(little, import(packet_format: u16))]
pub struct F1PacketCarDamage {
    /// Car damage data. Should have a size of 22.
    #[br(count(MAX_NUM_CARS), args{ inner: (packet_format,) })]
    pub car_damage_data: Vec<CarDamageData>,
}

/// Packet detailing lap and tyre data history for a given driver in the session
#[non_exhaustive]
#[derive(
    BinRead, PartialEq, PartialOrd, Clone, Debug, Serialize, Deserialize,
)]
#[br(
    little,
    import(packet_format: u16),
    assert(
        vehicle_index < MAX_NUM_CARS,
        "Session history packet has an invalid vehicle index: {}",
        vehicle_index
    ),
    assert(
        num_laps <= 100,
        "Session history packet has an invalid number of laps: {}",
        num_laps
    ),
    assert(
        num_tyre_stints <= 8,
        "Session history packet has an invalid number of tyre stints: {}",
        num_tyre_stints
    )
)]
pub struct F1PacketSessionHistory {
    /// Index of the car this packet refers to
    #[br(map(u8_to_usize))]
    pub vehicle_index: usize,
    /// Number of laps in the data (including the current one)
    #[br(map(u8_to_usize))]
    pub num_laps: usize,
    /// Number of tyre stints in the data (including the current one)
    #[br(map(u8_to_usize))]
    pub num_tyre_stints: usize,
    /// Number of the lap the best lap time was achieved on
    #[br(map(u8_to_usize))]
    pub best_lap_time_lap_num: usize,
    /// Number of the lap the best sector 1 time was achieved on
    #[br(map(u8_to_usize))]
    pub best_sector1_lap_num: usize,
    /// Number of the lap the best sector 2 time was achieved on
    #[br(map(u8_to_usize))]
    pub best_sector2_lap_num: usize,
    /// Number of the lap the best sector 3 time was achieved on
    #[br(map(u8_to_usize))]
    pub best_sector3_lap_num: usize,
    /// Up to 100 laps
    #[br(count(100), args{ inner: (packet_format,) })]
    pub lap_history_data: Vec<LapHistoryData>,
    /// Up to 8 tyre stints
    #[br(count(8), args{ inner: (packet_format,) })]
    pub tyre_stint_history_data: Vec<TyreStintHistoryData>,
}

/// In-depth details about tyre sets assigned to a vehicle during the session.
/// Available from the 2023 format onwards.
#[non_exhaustive]
#[derive(
    BinRead, PartialEq, PartialOrd, Clone, Debug, Serialize, Deserialize,
)]
#[br(little, import(packet_format: u16))]
pub struct F1PacketTyreSets {
    /// Index of the car this packet relates to.
    #[br(map(u8_to_usize))]
    pub vehicle_index: usize,
    /// 13 dry + 7 wet tyre sets.
    #[br(count(20), args{ inner: (packet_format,) })]
    pub tyre_set_data: Vec<TyreSetData>,
    /// Index of fitted tyre set.
    #[br(map(u8_to_usize))]
    pub fitted_index: usize,
}

/// Extended motion data for player's car. Available as a:
/// - part of [`F1PacketMotion`] in the 2022 format
/// - standalone packet from the 2023 format onwards
#[non_exhaustive]
#[derive(
    BinRead, PartialEq, PartialOrd, Copy, Clone, Debug, Serialize, Deserialize,
)]
#[br(little, import(packet_format: u16))]
pub struct F1PacketMotionEx {
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
    /// Vertical forces for each wheel.
    /// See [`wheel_index`](mod@crate::constants::wheel_index)
    /// for wheel order.
    /// Available from the 2023 format onwards.
    #[br(if(packet_format >= 2023))]
    pub wheel_vert_force: [f32; 4],
}

#[derive(Debug, PartialEq)]
pub(crate) struct InvalidBoolValue(u8);

impl core::fmt::Display for InvalidBoolValue {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Invalid bool value: {}", self.0)
    }
}

pub(crate) fn u8_to_bool(value: u8) -> Result<bool, InvalidBoolValue> {
    match value {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(InvalidBoolValue(value)),
    }
}

pub(crate) fn u8_to_usize(value: u8) -> usize {
    value as usize
}

pub(crate) fn read_name(bytes: [u8; 48]) -> Result<String, FromUtf8Error> {
    let first_nul_index =
        bytes.iter().position(|&byte| byte == b'\0').unwrap_or(bytes.len());

    String::from_utf8(bytes[..first_nul_index].to_vec())
}
