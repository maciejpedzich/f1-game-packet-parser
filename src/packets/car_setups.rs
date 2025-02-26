use binrw::BinRead;
use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(BinRead, PartialEq, PartialOrd, Copy, Clone, Debug, Serialize, Deserialize)]
#[br(little, import(_packet_format: u16))]
pub struct CarSetupData {
    /// Front wing aero.
    pub front_wing: u8,
    /// Rear wing aero.
    pub rear_wing: u8,
    /// Differential adjustment on throttle (percentage).
    #[br(
        assert(
            on_throttle <= 100,
            "Car setup entry has an invalid on-throttle percentage value: {}",
            on_throttle
        )
    )]
    pub on_throttle: u8,
    /// Differential adjustment off throttle (percentage).
    #[br(
        assert(
            off_throttle <= 100,
            "Car setup entry has an invalid off-throttle percentage value: {}",
            off_throttle
        )
    )]
    pub off_throttle: u8,
    /// Front camber angle (suspension geometry).
    pub front_camber: f32,
    /// Rear camber angle (suspension geometry).
    pub rear_camber: f32,
    /// Front toe angle (suspension geometry).
    pub front_toe: f32,
    /// Rear toe angle (suspension geometry).
    pub rear_toe: f32,
    /// Front suspension.
    pub front_suspension: u8,
    /// Rear suspension.
    pub rear_suspension: u8,
    /// Front anti-roll bar.
    pub front_anti_roll_bar: u8,
    /// Rear anti-roll bar.
    pub rear_anti_roll_bar: u8,
    /// Front ride height.
    pub front_suspension_height: u8,
    /// Rear ride height.
    pub rear_suspension_height: u8,
    /// Brake pressure (percentage).
    #[br(
        assert(
            brake_pressure <= 100,
            "Car setup entry has an invalid brake pressure percentage value: {}",
            brake_pressure
        )
    )]
    pub brake_pressure: u8,
    /// Brake bias.
    pub brake_bias: u8,
    /// Rear left tyre pressure.
    pub rear_left_tyre_pressure: f32,
    /// Rear right tyre pressure.
    pub rear_right_tyre_pressure: f32,
    /// Front left tyre pressure.
    pub front_left_tyre_pressure: f32,
    /// Front right tyre pressure.
    pub front_right_tyre_pressure: f32,
    /// Ballast.
    pub ballast: u8,
    /// Fuel load.
    pub fuel_load: f32,
}
