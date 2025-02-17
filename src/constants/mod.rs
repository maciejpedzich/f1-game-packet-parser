pub mod wheel_index;

use binrw::BinRead;
use bitflags::bitflags;
use serde::{Deserialize, Serialize};

pub(crate) const MAX_NUM_CARS: usize = 22;
pub(crate) const MAX_NUM_MARSHAL_ZONES: usize = 21;
pub(crate) const MAX_NUM_WEATHER_FORECAST_SAMPLES: usize = 56;
pub(crate) const MAX_AI_DIFFICULTY: u8 = 110;

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
#[br(little, repr(u8))]
pub enum PacketId {
    Motion = 0,
    Session = 1,
    LapData = 2,
    Event = 3,
    Participants = 4,
    CarSetups = 5,
    CarTelemetry = 6,
    CarStatus = 7,
    FinalClassification = 8,
    LobbyInfo = 9,
    CarDamage = 10,
    SessionHistory = 11,
    TyreSets = 12,
    MotionEx = 13,
    TimeTrial = 14,
}

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
#[br(little, repr(i8))]
pub enum MarshalZoneFlag {
    Unknown = -1,
    None = 0,
    Green = 1,
    Blue = 2,
    Yellow = 3,
    Red = 4,
}

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
#[br(little, repr(u8))]
pub enum Weather {
    Clear = 0,
    LightCloud = 1,
    Overcast = 2,
    LightRain = 3,
    HeavyRain = 4,
    Storm = 5,
}

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
#[br(little, repr(u8))]
pub enum SessionType {
    Unknown = 0,
    Practice1 = 1,
    Practice2 = 2,
    Practice3 = 3,
    ShortPractice = 4,
    Qualifying1 = 5,
    Qualifying2 = 6,
    Qualifying3 = 7,
    ShortQualifying = 8,
    OneShotQualifying = 9,
    Race = 10,
    Race2 = 11,
    TimeTrial = 12,
}

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
#[br(little, repr(i8))]
pub enum TemperatureChange {
    Up = 0,
    Down = 1,
    NoChange = 2,
}

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
#[br(little, repr(i8))]
pub enum TrackId {
    Unknown = -1,
    Melbourne = 0,
    PaulRicard = 1,
    Shanghai = 2,
    Sakhir = 3,
    Catalunya = 4,
    Monaco = 5,
    Montreal = 6,
    Silverstone = 7,
    Hockenheim = 8,
    Hungaroring = 9,
    Spa = 10,
    Monza = 11,
    Singapore = 12,
    Suzuka = 13,
    AbuDhabi = 14,
    Texas = 15,
    Brazil = 16,
    Austria = 17,
    Sochi = 18,
    Mexico = 19,
    Baku = 20,
    SakhirShort = 21,
    SilverstoneShort = 22,
    TexasShort = 23,
    SuzukaShort = 24,
    Hanoi = 25,
    Zandvoort = 26,
    Imola = 27,
    Portimao = 28,
    Jeddah = 29,
    Miami = 30,
    LasVegas = 31,
    Losail = 32,
}

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
#[br(little, repr(u8))]
pub enum Formula {
    F1Modern = 0,
    F1Classic = 1,
    F2 = 2,
    F1Generic = 3,
    Beta = 4,
    Supercars = 5,
    Esports = 6,
    F22021 = 7,
}

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
#[br(little, repr(u8))]
pub enum SafetyCarStatus {
    None = 0,
    Virtual = 1,
    Full = 2,
    FormationLap = 3,
}

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
#[br(little, repr(u8))]
pub enum ForecastAccuracy {
    Perfect = 0,
    Approximate = 1,
}

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
#[br(little, repr(u8))]
pub enum BrakingAssist {
    Off = 0,
    Low = 1,
    Medium = 2,
    High = 3,
}

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
#[br(little, repr(u8))]
pub enum GearboxAssist {
    Unknown = 0,
    Manual = 1,
    ManualWithSuggestedGear = 2,
    Automatic = 3,
}

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
#[br(little, repr(u8))]
pub enum DynamicRacingLine {
    Off = 0,
    CornersOnly = 1,
    Full = 2,
}

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
#[br(little, repr(u8))]
pub enum DynamicRacingLineType {
    TwoDimensional = 0,
    ThreeDimensional = 1,
}

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
#[br(little, repr(u8))]
pub enum GameMode {
    EventMode = 0,
    GrandPrix = 3,
    TimeTrial = 5,
    Splitscreen = 6,
    OnlineCustom = 7,
    OnlineLeague = 8,
    CareerInvitational = 11,
    ChampionshipInvitational = 12,
    Championship = 13,
    OnlineChampionship = 14,
    OnlineWeeklyEvent = 15,
    Career2022 = 19,
    Career2022Online = 20,
    Benchmark = 127,
}

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
#[br(little, repr(u8))]
pub enum Ruleset {
    PracticeAndQualifying = 0,
    Race = 1,
    TimeTrial = 2,
    TimeAttack = 4,
    CheckpointChallenge = 6,
    Autocross = 8,
    Drift = 9,
    AverageSpeedZone = 10,
    RivalDuel = 11,
}

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
#[br(little, repr(u8))]
pub enum SessionLength {
    None = 0,
    VeryShort = 2,
    Short = 3,
    Medium = 4,
    MediumLong = 5,
    Long = 6,
    Full = 7,
}

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
#[br(little, repr(u8))]
pub enum PitStatus {
    None = 0,
    Pitting = 1,
    InPitArea = 2,
}

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
#[br(little, repr(u8))]
pub enum DriverStatus {
    InGarage = 0,
    FlyingLap = 1,
    InLap = 2,
    OutLap = 3,
    OnTrack = 4,
}

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
#[br(little, repr(u8))]
pub enum ResultStatus {
    Unknown = 0,
    Inactive = 1,
    Active = 2,
    Finished = 3,
    DidNotFinish = 4,
    Disqualified = 5,
    NotClassified = 6,
    Retired = 7,
}

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
#[br(little, repr(u8))]
pub enum PenaltyType {
    DriveThrough = 0,
    StopGo = 1,
    GridPenalty = 2,
    PenaltyReminder = 3,
    TimePenalty = 4,
    Warning = 5,
    Disqualified = 6,
    RemovedFromFormationLap = 7,
    ParkedTooLongTimer = 8,
    TyreRegulations = 9,
    ThisLapInvalidated = 10,
    ThisAndNextLapInvalidated = 11,
    ThisLapInvalidatedWithoutReason = 12,
    ThisAndNextLapInvalidatedWithoutReason = 13,
    ThisAndPreviousLapInvalidated = 14,
    ThisAndPreviousLapInvalidatedWithoutReason = 15,
    Retired = 16,
    BlackFlagTimer = 17,
}

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
#[br(little, repr(u8))]
pub enum InfringementType {
    BlockingBySlowDriving = 0,
    BlockingByWrongWayDriving = 1,
    ReversingOffTheStartLine = 2,
    BigCollision = 3,
    SmallCollision = 4,
    CollisionFailedToHandBackPositionSingle = 5,
    CollisionFailedToHandBackPositionMultiple = 6,
    CornerCuttingGainedTime = 7,
    CornerCuttingOvertakeSingle = 8,
    CornerCuttingOvertakeMultiple = 9,
    CrossedPitExitLane = 10,
    IgnoringBlueFlags = 11,
    IgnoringYellowFlags = 12,
    IgnoringDriveThrough = 13,
    TooManyDriveThroughs = 14,
    DriveThroughReminderServeWithinNLaps = 15,
    DriveThroughReminderServeThisLap = 16,
    PitLaneSpeeding = 17,
    ParkedForTooLong = 18,
    IgnoringTyreRegulations = 19,
    TooManyPenalties = 20,
    MultipleWarnings = 21,
    ApproachingDisqualification = 22,
    TyreRegulationsSelectSingle = 23,
    TyreRegulationsSelectMultiple = 24,
    LapInvalidatedCornerCutting = 25,
    LapInvalidatedRunningWide = 26,
    CornerCuttingRanWideMinorTimeGain = 27,
    CornerCuttingRanWideSignificantTimeGain = 28,
    CornerCuttingRanWideExtremeTimeGain = 29,
    LapInvalidatedWallRiding = 30,
    LapInvalidatedFlashbackUsed = 31,
    LapInvalidatedResetToTrack = 32,
    BlockingThePitLane = 33,
    JumpStart = 34,
    SafetyCarCollision = 35,
    SafetyCarIllegalOvertake = 36,
    SafetyCarExceedingAllowedPace = 37,
    VirtualSafetyCarExceedingAllowedPace = 38,
    FormationLapBelowAllowedSpeed = 39,
    RetiredMechanicalFailure = 40,
    RetiredTerminallyDamaged = 41,
    SafetyCarFallingTooFarBack = 42,
    BlackFlagTimer = 43,
    UnservedStopGoPenalty = 44,
    UnservedDriveThroughPenalty = 45,
    EngineComponentChange = 46,
    GearboxChange = 47,
    LeagueGridPenalty = 48,
    RetryPenalty = 49,
    IllegalTimeGain = 50,
    MandatoryPitStop = 51,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Ord,
    PartialOrd,
    Hash,
    Serialize,
    Deserialize,
)]
pub struct ButtonStatus(u32);

bitflags! {
    impl ButtonStatus: u32 {
        /// The "A" button on an Xbox controller
        /// or the cross button on a PlayStation controller.
        /// Has a value of `0x00000001`.
        const A_OR_CROSS = 0x00000001;
        /// The "Y" button on an Xbox controller
        /// or the triangle button on a PlayStation controller.
        /// Has a value of `0x00000002`.
        const Y_OR_TRIANGLE = 0x00000002;
        /// The "B" button on an Xbox controller
        /// or the circle button on a PlayStation controller.
        /// Has a value of `0x00000004`.
        const B_OR_CIRCLE = 0x00000004;
        /// The "X" button on an Xbox controller
        /// or the square button on a PlayStation controller.
        /// Has a value of `0x00000008`.
        const X_OR_SQUARE = 0x00000008;
        /// Left directional pad button. Has a value of `0x00000010`.
        const DPAD_LEFT = 0x00000010;
        /// Right directional pad button. Has a value of `0x00000020`.
        const DPAD_RIGHT = 0x00000020;
        /// Up directional pad button. Has a value of `0x00000040`.
        const DPAD_UP = 0x00000040;
        /// Down directional pad button. Has a value of `0x00000080`.
        const DPAD_DOWN = 0x00000080;
        /// The "Menu" button on an Xbox controller
        /// or the "Options" button on a PlayStation controller.
        /// Has a value of `0x00000100`.
        const MENU_OR_OPTIONS = 0x00000100;
        /// Left bumper. Has a value of `0x00000200`.
        const LEFT_BUMPER = 0x00000200;
        /// Right bumper. Has a value of `0x00000400`.
        const RIGHT_BUMPER = 0x00000400;
        /// Left trigger. Has a value of `0x00000800`.
        const LEFT_TRIGGER = 0x00000800;
        /// Right trigger. Has a value of `0x00001000`.
        const RIGHT_TRIGGER = 0x00001000;
        /// Left analog stick click. Has a value of `0x00002000`.
        const LEFT_STICK_CLICK = 0x00002000;
        /// Right analog stick click. Has a value of `0x00004000`.
        const RIGHT_STICK_CLICK = 0x00004000;
        /// Right analog stick left. Has a value of `0x00008000`.
        const RIGHT_STICK_LEFT = 0x00008000;
        /// Right analog stick right. Has a value of `0x00010000`
        const RIGHT_STICK_RIGHT = 0x00010000;
        /// Right analog stick up. Has a value of `0x00020000`
        const RIGHT_STICK_UP = 0x00020000;
        /// Right analog stick down. Has a value of `0x00040000`
        const RIGHT_STICK_DOWN = 0x00040000;
        /// Special button. Has a value of `0x00080000`.
        const SPECIAL = 0x00080000;
        /// UDP Action 1. Has a value of `0x00100000`.
        const UDP_ACTION_1 = 0x00100000;
        /// UDP Action 2. Has a value of `0x00200000`.
        const UDP_ACTION_2 = 0x00200000;
        /// UDP Action 3. Has a value of `0x00400000`.
        const UDP_ACTION_3 = 0x00400000;
        /// UDP Action 4. Has a value of `0x00800000`.
        const UDP_ACTION_4 = 0x00800000;
        /// UDP Action 5. Has a value of `0x01000000`.
        const UDP_ACTION_5 = 0x01000000;
        /// UDP Action 6. Has a value of `0x02000000`.
        const UDP_ACTION_6 = 0x02000000;
        /// UDP Action 7. Has a value of `0x04000000`.
        const UDP_ACTION_7 = 0x04000000;
        /// UDP Action 8. Has a value of `0x08000000`.
        const UDP_ACTION_8 = 0x08000000;
        /// UDP Action 9. Has a value of `0x10000000`.
        const UDP_ACTION_9 = 0x10000000;
        /// UDP Action 10. Has a value of `0x20000000`.
        const UDP_ACTION_10 = 0x20000000;
        /// UDP Action 11. Has a value of `0x40000000`.
        const UDP_ACTION_11 = 0x40000000;
        /// UDP Action 12. Has a value of `0x80000000`.
        const UDP_ACTION_12 = 0x80000000;
    }
}
