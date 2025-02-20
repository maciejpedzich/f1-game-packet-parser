use super::u8_to_bool;
use crate::constants::{ActualTyreCompound, SessionType, VisualTyreCompound};

use binrw::BinRead;
use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(
    BinRead,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Copy,
    Clone,
    Debug,
    Serialize,
    Deserialize,
)]
#[br(
    little,
    import(_packet_format: u16),
    assert(
        wear <= 100,
        "Tyre set entry has an invalid wear percentage: {}",
        wear
    )
)]
pub struct TyreSetData {
    /// Actual tyre compound.
    pub actual_tyre_compound: ActualTyreCompound,
    /// Visual tyre compound.
    pub visual_tyre_compound: VisualTyreCompound,
    /// Tyre wear (percentage).
    pub wear: u8,
    /// Whether this set is currently available.
    #[br(try_map(u8_to_bool))]
    pub available: bool,
    /// Recommended session for this tyre set.
    pub recommended_session: SessionType,
    /// Laps left in this set.
    pub life_span: u8,
    /// Max number of laps recommended for this compound.
    pub usable_life: u8,
    /// Lap time delta in milliseconds compared to fitted set.
    pub lap_delta_time: i16,
    /// Whether this set is fitted or not.
    #[br(try_map(u8_to_bool))]
    pub fitted: bool,
}
