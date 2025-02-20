use super::u8_to_bool;
use binrw::BinRead;
use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(
    BinRead, PartialEq, PartialOrd, Copy, Clone, Debug, Serialize, Deserialize,
)]
#[br(little, import(_packet_format: u16))]
pub struct CarDamageData {
    /// Tyre wear percentage for all wheels.
    /// See [`wheel_index`](mod@crate::constants::wheel_index)
    /// for wheel order.
    pub tyres_wear: [f32; 4],
    /// Tyre damage percentage for all wheels.
    /// See [`wheel_index`](mod@crate::constants::wheel_index)
    /// for wheel order.
    pub tyres_damage: [u8; 4],
    /// Brake damage percentage for all wheels.
    /// See [`wheel_index`](mod@crate::constants::wheel_index)
    /// for wheel order.
    pub brakes_damage: [u8; 4],
    /// Front left wing damage (percentage)
    pub front_left_wing_damage: u8,
    /// Front right wing damage (percentage)
    pub front_right_wing_damage: u8,
    /// Rear wing damage (percentage)
    pub rear_wing_damage: u8,
    /// Floor damage (percentage)
    pub floor_damage: u8,
    /// Diffuser damage (percentage)
    pub diffuser_damage: u8,
    /// Sidepod damage (percentage)
    pub sidepod_damage: u8,
    /// Whether DRS has failed
    #[br(try_map(u8_to_bool))]
    pub drs_fault: bool,
    /// Whether ERS has failed
    #[br(try_map(u8_to_bool))]
    pub ers_fault: bool,
    /// Gearbox damage (percentage)
    pub gearbox_damage: u8,
    /// Engine damage (percentage)
    pub engine_damage: u8,
    /// Engine MGU-H wear (percentage)
    pub engine_mguh_wear: u8,
    /// Engine ES wear (percentage)
    pub engine_es_wear: u8,
    /// Engine CE wear (percentage)
    pub engine_ce_wear: u8,
    /// Engine ICE wear (percentage)
    pub engine_ice_wear: u8,
    /// Engine MGU-K wear (percentage)
    pub engine_mguk_wear: u8,
    /// Engine TC wear (percentage)
    pub engine_tc_wear: u8,
    /// Whether the engine has blown
    #[br(try_map(u8_to_bool))]
    pub engine_blown: bool,
    /// Whether the engine has seized
    #[br(try_map(u8_to_bool))]
    pub engine_seized: bool,
}
