use binrw::BinRead;
use serde::{Deserialize, Serialize};

#[derive(
    BinRead, PartialEq, PartialOrd, Copy, Clone, Debug, Serialize, Deserialize,
)]
#[br(little)]
pub struct CarMotionData {
    /// World space X position in metres
    pub world_position_x: f32,
    /// World space Y position in metres
    pub world_position_y: f32,
    /// World space Z position in metres
    pub world_position_z: f32,
    /// Velocity in world space X position
    pub world_velocity_x: f32,
    /// Velocity in world space Y position
    pub world_velocity_y: f32,
    /// Velocity in world space Z position
    pub world_velocity_z: f32,
    /// World space forward X direction (normalised)
    pub world_forward_dir_x: i16,
    /// World space forward Y direction (normalised)
    pub world_forward_dir_y: i16,
    /// World space forward Z direction (normalised)
    pub world_forward_dir_z: i16,
    /// World space right X direction (normalised)
    pub world_right_dir_x: i16,
    /// World space right Y direction (normalised)
    pub world_right_dir_y: i16,
    /// World space right Z direction (normalised)
    pub world_right_dir_z: i16,
    /// Lateral G-Force component
    pub g_force_lateral: f32,
    /// Longitudinal G-Force component
    pub g_force_longitudinal: f32,
    /// Vertical G-Force component
    pub g_force_vertical: f32,
    /// Yaw angle in radians
    pub yaw: f32,
    /// Pitch angle in radians
    pub pitch: f32,
    /// Roll angle in radians
    pub roll: f32,
}

impl CarMotionData {
    /// Returns [`self.world_forward_dir_x`] divided by `32767.0f32`
    pub fn world_forward_dir_x_as_f32(&self) -> f32 {
        self.world_forward_dir_x as f32 / 32767.0
    }

    /// Returns [`self.world_forward_dir_y`] divided by `32767.0f32`
    pub fn world_forward_dir_y_as_f32(&self) -> f32 {
        self.world_forward_dir_y as f32 / 32767.0
    }

    /// Returns [`self.world_forward_dir_z`] divided by `32767.0f32`
    pub fn world_forward_dir_z_as_f32(&self) -> f32 {
        self.world_forward_dir_z as f32 / 32767.0
    }

    /// Returns [`self.world_right_dir_x`] divided by `32767.0f32`
    pub fn world_right_dir_x_as_f32(&self) -> f32 {
        self.world_right_dir_x as f32 / 32767.0
    }

    /// Returns [`self.world_right_dir_y`] divided by `32767.0f32`
    pub fn world_right_dir_y_as_f32(&self) -> f32 {
        self.world_right_dir_y as f32 / 32767.0
    }

    /// Returns [`self.world_right_dir_z`] divided by `32767.0f32`
    pub fn world_right_dir_z_as_f32(&self) -> f32 {
        self.world_right_dir_z as f32 / 32767.0
    }
}
