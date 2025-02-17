pub mod motion;

use crate::packets::motion::CarMotionData;

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
    #[br(count(22))]
    pub car_motion_data: Vec<CarMotionData>,
    /// Extra player car ONLY motion data (2022 format only).
    #[br(if(packet_format == 2022))]
    pub motion_ex_data: Option<F1PacketMotionExData>,
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
