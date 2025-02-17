pub mod wheel_index;

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
