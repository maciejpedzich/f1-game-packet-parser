use crate::constants::{MarshalZoneFlag, SessionType, TemperatureChange, Weather};
use binrw::BinRead;
use serde::{Deserialize, Serialize};

pub(super) const MAX_NUM_MARSHAL_ZONES: usize = 21;
pub(super) const MARSHAL_ZONE_RAW_SIZE: usize = 5;
pub(super) const FORECAST_SAMPLE_RAW_SIZE: usize = 8;
pub(super) const MAX_AI_DIFFICULTY: u8 = 110;
pub(super) const MAX_NUM_SESSIONS: usize = 12;

/// Section of the track supervised by marshals.
#[non_exhaustive]
#[derive(BinRead, PartialEq, PartialOrd, Copy, Clone, Debug, Serialize, Deserialize)]
#[br(
    little,
    import(_packet_format: u16),
    assert(
        (0.0..1.0).contains(&zone_start),
        "Marshal zone has an invalid zone start value: {}",
        zone_start
    )
)]
pub struct MarshalZone {
    /// Fraction (in range `(0.0..1.0)`) of way through the lap the marshal zone starts.
    pub zone_start: f32,
    /// Flag that's currently being waved in the marshal zone.
    pub zone_flag: MarshalZoneFlag,
}

#[non_exhaustive]
/// Weather forecast sample for a given session.
#[derive(
    BinRead, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Debug, Serialize, Deserialize,
)]
#[br(
    little,
    import(_packet_format: u16),
    assert(
        rain_percentage <= 100,
        "Weather forecast sample has an invalid rain percentage value: {}",
        rain_percentage
    )
)]
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

#[inline(always)]
pub(super) fn check_num_forecast_samples(packet_format: u16, num_samples: usize) -> bool {
    num_samples <= get_max_num_samples(packet_format)
}

#[inline(always)]
pub(super) fn get_forecast_samples_padding(
    packet_format: u16,
    num_samples: usize,
) -> usize {
    (get_max_num_samples(packet_format) - num_samples) * FORECAST_SAMPLE_RAW_SIZE
}

#[inline(always)]
fn get_max_num_samples(packet_format: u16) -> usize {
    if packet_format == 2024 {
        64
    } else {
        56
    }
}
