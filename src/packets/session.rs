use crate::constants::{
    MarshalZoneFlag, SessionType, TemperatureChange, Weather,
};
use binrw::BinRead;
use serde::{Deserialize, Serialize};

/// Section of the track supervised by marshals.
#[derive(
    BinRead, PartialEq, PartialOrd, Copy, Clone, Debug, Serialize, Deserialize,
)]
#[br(little, import(_packet_format: u16))]
pub struct MarshalZone {
    /// Fraction (0..1) of way through the lap the marshal zone starts.
    pub zone_start: f32,
    /// Flag that's currently being waved in the marshal zone.
    pub zone_flag: MarshalZoneFlag,
}

/// Weather forecast sample for a given session.
#[derive(
    BinRead, PartialEq, PartialOrd, Copy, Clone, Debug, Serialize, Deserialize,
)]
#[br(little, import(_packet_format: u16))]
pub struct WeatherForecastSample {
    /// Session's type.
    pub session_type: SessionType,
    /// Time in minutes the forecast is for.
    pub time_offset: u8,
    /// Forecasted weather.
    pub weather: Weather,
    /// Track temperature in degrees Celsius.
    pub track_temperature: i8,
    /// Track temperature change.
    pub track_temperature_change: TemperatureChange,
    /// Air temperature in degrees Celsius.
    pub air_temperature: i8,
    /// Air temperature change.
    pub air_temperature_change: TemperatureChange,
    /// Chance of rain.
    pub rain_percentage: u8,
}
