use super::u8_to_usize;
use crate::constants::{ButtonStatus, InfringementType, PenaltyType};

use binrw::BinRead;
use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(
    BinRead, PartialEq, PartialOrd, Clone, Debug, Serialize, Deserialize,
)]
#[br(little)]
pub enum EventDataDetails {
    /// Sent when the session starts.
    #[br(magic = b"SSTA")]
    SessionStarted,
    /// Sent when the session ends.
    #[br(magic = b"SEND")]
    SessionEnded,
    /// Sent when a driver achieves the fastest lap.
    #[br(magic = b"FTLP")]
    FastestLap {
        /// Index of the car that's achieved the fastest lap.
        #[br(map(u8_to_usize))]
        vehicle_index: usize,
        /// Lap time in seconds.
        lap_time: f32,
    },
    /// Sent when a driver retires.
    #[br(magic = b"RTMT")]
    Retirement {
        /// Index of the retiring car.
        #[br(map(u8_to_usize))]
        vehicle_index: usize,
    },
    /// Sent when race control enable DRS.
    #[br(magic = b"DRSE")]
    DrsEnabled,
    /// Sent when race control disable DRS.
    #[br(magic = b"DRSD")]
    DrsDisabled,
    /// Sent when your teammate enters the pit lane.
    #[br(magic = b"TMPT")]
    TeamMateInPits {
        /// Index of teammate's car.
        #[br(map(u8_to_usize))]
        vehicle_index: usize,
    },
    /// Sent when the chequered flag has been waved.
    #[br(magic = b"CHQF")]
    ChequeredFlag,
    /// Sent when the race winner is announced.
    #[br(magic = b"RCWN")]
    RaceWinner {
        /// Index of race winner's car.
        #[br(map(u8_to_usize))]
        vehicle_index: usize,
    },
    /// Sent when a penalty has been issued.
    #[br(magic = b"PENA")]
    Penalty {
        /// Penalty type.
        penalty_type: PenaltyType,
        /// Infringement type.
        infringement_type: InfringementType,
        /// Index of the car the penalty is applied to.
        #[br(map(u8_to_usize))]
        vehicle_index: usize,
        /// Index of the other car involved.
        #[br(map(u8_to_usize))]
        other_vehicle_index: usize,
        /// Time gained/spent doing the action in seconds.
        time: u8,
        /// Number of the lap the infringement occurred on.
        lap_num: u8,
        /// Number of places gained by this infringement.
        places_gained: u8,
    },
    /// Sent when a speed trap is triggered.
    #[br(magic = b"SPTP")]
    SpeedTrap {
        /// Index of the car that's triggered the speed trap.
        #[br(map(u8_to_usize))]
        vehicle_index: usize,
        /// Top speed achieved in kilometres per hour.
        speed: f32,
    },
    /// Sent when a start light is lit.
    #[br(magic = b"STLG")]
    StartLights {
        /// Number of lights showing.
        num_lights: u8,
    },
    /// "It's lights out, and away we go!"
    #[br(magic = b"LGOT")]
    LightsOut,
    /// Sent when a driver has served a drive-through penalty.
    #[br(magic = b"DTSV")]
    DriveThroughServed {
        /// Index of the vehicle serving the penalty.
        #[br(map(u8_to_usize))]
        vehicle_index: usize,
    },
    /// Sent when a driver has served a stop-go penalty.
    #[br(magic = b"SGSV")]
    StopGoServed {
        /// Index of the vehicle serving the penalty.
        #[br(map(u8_to_usize))]
        vehicle_index: usize,
    },
    /// Sent when a flashback is activated.
    #[br(magic = b"FLBK")]
    Flashback {
        /// Frame identifier that's been flashed back to.
        frame_identifier: u32,
        /// Session time that's been flashed back to.
        flashback_session_time: f32,
    },
    /// Sent when the button status has changed.
    #[br(magic = b"BUTN")]
    Buttons {
        /// Bit flags specifying which buttons are currently pressed.
        #[br(map(|bits: u32| ButtonStatus::from_bits_truncate(bits)))]
        button_status: ButtonStatus
    },
}
