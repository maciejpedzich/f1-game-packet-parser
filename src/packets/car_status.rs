use super::u8_to_bool;
use crate::constants::{
    ActualTyreCompound, DrsAllowed, ErsDeployMode, FuelMix, TractionControl,
    VehicleFiaFlag, VisualTyreCompound,
};

use binrw::BinRead;
use serde::{Deserialize, Serialize};

#[derive(
    BinRead, PartialEq, PartialOrd, Copy, Clone, Debug, Serialize, Deserialize,
)]
#[br(
    little,
    import(packet_format: u16),
    assert(
        max_gears <= 9,
        "Car status entry has an invalid max number of gears: {}",
        max_gears
    )
)]
pub struct CarStatusData {
    /// How much traction control is enabled.
    pub traction_control: TractionControl,
    /// Whether ABS is enabled.
    #[br(map(u8_to_bool))]
    pub anti_lock_brakes: bool,
    /// Fuel mix currently in use.
    pub fuel_mix: FuelMix,
    /// Front brake bias (percentage).
    pub front_brake_bias: u8,
    /// Whether the pit limiter is enabled.
    #[br(map(u8_to_bool))]
    pub pit_limiter_enabled: bool,
    /// Current fuel mass.
    pub fuel_in_tank: f32,
    /// Fuel capacity.
    pub fuel_capacity: f32,
    /// Fuel remaining in terms of laps.
    pub fuel_remaining_laps: f32,
    /// Car's max RPM, point of rev limiter.
    pub max_rpm: u16,
    /// Car's idle RPM.
    pub idle_rpm: u16,
    /// Maximum number of gears.
    pub max_gears: u8,
    /// Whether DRS can be used (might be unknown).
    pub drs_allowed: DrsAllowed,
    /// 0 = DRS is unavailable, Non-zero = DRS will be available in X metres.
    pub drs_activation_distance: u16,
    /// Actual tyre compound currently in use.
    pub actual_tyre_compound: ActualTyreCompound,
    /// Visible tyre compound currently in use.
    pub visual_tyre_compound: VisualTyreCompound,
    /// Age of the current set of tyres in laps.
    pub tyres_age_laps: u8,
    /// Flag the driver is currently being shown.
    pub vehicle_fia_flag: VehicleFiaFlag,
    /// Engine power output of ICE in watts.
    /// Available from the 2023 format onwards.
    #[br(if(packet_format >= 2023))]
    pub engine_power_ice: f32,
    /// Engine power output of MGU-K in watts.
    /// Available from the 2023 format onwards.
    #[br(if(packet_format >= 2023))]
    pub engine_power_mguk: f32,
    /// ERS energy store in Joules.
    pub ers_store_energy: f32,
    /// ERS deployment mode.
    pub ers_deploy_mode: ErsDeployMode,
    /// ERS energy harvested this lap by the MGU-K.
    pub ers_harvested_this_lap_mguk: f32,
    /// ERS energy harvested this lap by the MGU-H.
    pub ers_harvested_this_lap_mguh: f32,
    /// ERS energy deployed this lap.
    pub ers_deployed_this_lap: f32,
    /// Whether the car has paused in a network game.
    #[br(map(u8_to_bool))]
    pub network_paused: bool,
}
