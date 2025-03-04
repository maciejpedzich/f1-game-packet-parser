/// Unique identifiers of drivers.
pub mod driver_id;
/// Unique identifiers of session types.
pub mod session_type;
/// Unique identifiers of teams.
pub mod team_id;
/// Indexes of wheels in wheel-oriented arrays.
/// The order is:
/// [`REAR_LEFT`](const@wheel_index::REAR_LEFT),
/// [`REAR_RIGHT`](const@wheel_index::REAR_RIGHT),
/// [`FRONT_LEFT`](const@wheel_index::FRONT_LEFT),
/// [`FRONT_RIGHT`](const@wheel_index::FRONT_RIGHT).
pub mod wheel_index;

use binrw::BinRead;
use bitflags::bitflags;
use serde::{Deserialize, Serialize};

pub(crate) const MAX_NUM_CARS: usize = 22;

/// Unique identifier of the type of this packet.
/// Represents a [`u8`].
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
    Hash,
    Serialize,
    Deserialize,
)]
#[br(little, repr(u8))]
pub enum PacketId {
    Motion = 0,
    Session = 1,
    Laps = 2,
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

/// Flag that's currently being waved in
/// a [`MarshalZone`](crate::packets::session::MarshalZone).
/// Represents an [`i8`].
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
    Hash,
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

/// Session/forecast weather type. Represents a [`u8`].
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
    Hash,
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

/// Temperature change direction. Represents an [`i8`].
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
    Hash,
    Serialize,
    Deserialize,
)]
#[br(little, repr(i8))]
pub enum TemperatureChange {
    Up = 0,
    Down = 1,
    NoChange = 2,
}

/// Unique circuit ID. Represents an [`i8`].
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
    Hash,
    Serialize,
    Deserialize,
)]
#[br(little, repr(i8))]
pub enum TrackId {
    /// Unknown circuit.
    Unknown = -1,
    /// Australian Grand Prix.
    AlbertPark = 0,
    /// French Grand Prix.
    PaulRicard = 1,
    /// Chinese Grand Prix.
    Shanghai = 2,
    /// Bahrain Grand Prix.
    Sakhir = 3,
    /// Spanish Grand Prix.
    Catalunya = 4,
    /// Monaco Grand Prix.
    MonteCarlo = 5,
    /// Canadian Grand Prix.
    Montreal = 6,
    /// British Grand Prix.
    Silverstone = 7,
    /// German Grand Prix.
    Hockenheim = 8,
    /// Hungarian Grand Prix.
    Hungaroring = 9,
    /// Belgian Grand Prix.
    Spa = 10,
    /// Italian Grand Prix.
    Monza = 11,
    /// Singapore Grand Prix.
    MarinaBay = 12,
    /// Japanese Grand Prix.
    Suzuka = 13,
    /// Abu Dhabi Grand Prix.
    YasMarina = 14,
    /// Circuit of the Americas. United States (Texas) Grand Prix.
    Cota = 15,
    /// Brazilian (Sao Paulo) Grand Prix.
    Interlagos = 16,
    /// Austrian Grand Prix.
    RedBullRing = 17,
    /// Russian Grand Prix.
    Sochi = 18,
    /// Mexican Grand Prix.
    MexicoCity = 19,
    /// Azerbaijan Grand Prix.
    Baku = 20,
    /// Short variant of the [`Sakhir`](TrackId::Sakhir) circuit.
    SakhirShort = 21,
    /// Short variant of the [`Silverstone`](TrackId::Silverstone) circuit.
    SilverstoneShort = 22,
    /// Short variant of the [`Cota`](TrackId::Cota) circuit.
    CotaShort = 23,
    /// Short variant of the [`Suzuka`](TrackId::Suzuka) circuit.
    SuzukaShort = 24,
    /// Vietnamese Grand Prix.
    Hanoi = 25,
    /// Dutch Grand Prix.
    Zandvoort = 26,
    /// ~~San Marino~~ Emilia-Romagna Grand Prix.
    Imola = 27,
    /// Portuguese Grand Prix.
    Portimao = 28,
    /// Saudi Arabian Grand Prix.
    Jeddah = 29,
    /// Miami Grand Prix.
    Miami = 30,
    /// Las Vegas Grand Prix.
    LasVegas = 31,
    /// Qatar Grand Prix.
    Losail = 32,
}

/// Type of cars being raced in
/// [`F1PacketSession`](struct@crate::F1PacketSession).
/// Represents a [`u8`].
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
    Hash,
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
    F1World = 8,
    F1Elimination = 9,
}

/// Safety car deployment status in [`F1PacketSession`](struct@crate::F1PacketSession).
/// Represents a [`u8`].
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
    Hash,
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

/// Accuracy of a
/// [`WeatherForecastSample`](struct@crate::packets::session::WeatherForecastSample).
/// Represents a [`u8`].
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
    Hash,
    Serialize,
    Deserialize,
)]
#[br(little, repr(u8))]
pub enum ForecastAccuracy {
    Perfect = 0,
    Approximate = 1,
}

/// Type of enabled braking assist. Represents a [`u8`].
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
    Hash,
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

/// Type of enabled gearbox assist. Represents a [`u8`].
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
    Hash,
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

/// Type of enabled racing line assist. Represents a [`u8`].
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
    Hash,
    Serialize,
    Deserialize,
)]
#[br(little, repr(u8))]
pub enum DynamicRacingLine {
    Off = 0,
    CornersOnly = 1,
    Full = 2,
}

/// Shape of the racing line. Represents a [`u8`].
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
    Hash,
    Serialize,
    Deserialize,
)]
#[br(little, repr(u8))]
pub enum DynamicRacingLineType {
    TwoDimensional = 0,
    ThreeDimensional = 1,
}

/// Game mode that's currently in use. Represents a [`u8`].
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
    Hash,
    Serialize,
    Deserialize,
)]
#[br(little, repr(u8))]
pub enum GameMode {
    EventMode = 0,
    GrandPrix = 3,
    GrandPrix2023 = 4,
    TimeTrial = 5,
    Splitscreen = 6,
    OnlineCustom = 7,
    OnlineLeague = 8,
    CareerInvitational = 11,
    ChampionshipInvitational = 12,
    Championship = 13,
    OnlineChampionship = 14,
    OnlineWeeklyEvent = 15,
    BrakingPoint2023 = 17,
    Career2022 = 19,
    OnlineCareer2022 = 20,
    Career2023 = 21,
    OnlineCareer2023 = 22,
    DriverCareer2024 = 23,
    OnlineCareer2024 = 24,
    MyTeamCareer2024 = 25,
    CuratedCareer2024 = 26,
    Benchmark = 127,
}

/// Set of rules that's in use for this session. Represents a [`u8`].
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
    Hash,
    Serialize,
    Deserialize,
)]
#[br(little, repr(u8))]
pub enum RuleSet {
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

/// Length of the ongoing session. Represents a [`u8`].
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
    Hash,
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

/// Whether the car is outside/entering/in the pit lane. Represents a [`u8`].
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
    Hash,
    Serialize,
    Deserialize,
)]
#[br(little, repr(u8))]
pub enum PitStatus {
    None = 0,
    Pitting = 1,
    InPitArea = 2,
}

/// Zero-based sector number. Represents a [`u8`].
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
    Hash,
    Serialize,
    Deserialize,
)]
#[br(little, repr(u8))]
pub enum Sector {
    First = 0,
    Second = 1,
    Third = 2,
}

/// Status of a driver in the current session. Represents a [`u8`].
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
    Hash,
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

/// Status of a driver's result in the current session and final classification.
/// Represents a [`u8`].
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
    Hash,
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

/// Type of penalty awarded to a driver. Represents a [`u8`].
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
    Hash,
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

/// Type of offence commited by a driver. Represents a [`u8`].
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
    Hash,
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
    FormationLapParking = 40,
    RetiredMechanicalFailure = 41,
    RetiredTerminallyDamaged = 42,
    SafetyCarFallingTooFarBack = 43,
    BlackFlagTimer = 44,
    UnservedStopGoPenalty = 45,
    UnservedDriveThroughPenalty = 46,
    EngineComponentChange = 47,
    GearboxChange = 48,
    ParcFermeChange = 49,
    LeagueGridPenalty = 50,
    RetryPenalty = 51,
    IllegalTimeGain = 52,
    MandatoryPitStop = 53,
    AttributeAssigned = 54,
}

/// Bit flags of specific controller buttons being pressed
/// in a [`Buttons` event](variant@crate::packets::event::EventDetails::Buttons).
/// Represents a [`u32`].
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Serialize, Deserialize,
)]
pub struct ButtonStatus(u32);

bitflags! {
    impl ButtonStatus: u32 {
        /// The "A" button on an Xbox controller
        /// or the cross button on a PlayStation controller.
        /// Has a value of `0x0000_0001`.
        const A_OR_CROSS = 0x0000_0001;
        /// The "Y" button on an Xbox controller
        /// or the triangle button on a PlayStation controller.
        /// Has a value of `0x0000_0002`.
        const Y_OR_TRIANGLE = 0x0000_0002;
        /// The "B" button on an Xbox controller
        /// or the circle button on a PlayStation controller.
        /// Has a value of `0x0000_0004`.
        const B_OR_CIRCLE = 0x0000_0004;
        /// The "X" button on an Xbox controller
        /// or the square button on a PlayStation controller.
        /// Has a value of `0x0000_0008`.
        const X_OR_SQUARE = 0x0000_0008;
        /// Left directional pad button. Has a value of `0x0000_0010`.
        const DPAD_LEFT = 0x0000_0010;
        /// Right directional pad button. Has a value of `0x0000_0020`.
        const DPAD_RIGHT = 0x0000_0020;
        /// Up directional pad button. Has a value of `0x0000_0040`.
        const DPAD_UP = 0x0000_0040;
        /// Down directional pad button. Has a value of `0x0000_0080`.
        const DPAD_DOWN = 0x0000_0080;
        /// The "Menu" button on an Xbox controller
        /// or the "Options" button on a PlayStation controller.
        /// Has a value of `0x0000_0100`.
        const MENU_OR_OPTIONS = 0x0000_0100;
        /// Left bumper. Has a value of `0x0000_0200`.
        const LEFT_BUMPER = 0x0000_0200;
        /// Right bumper. Has a value of `0x0000_0400`.
        const RIGHT_BUMPER = 0x0000_0400;
        /// Left trigger. Has a value of `0x0000_0800`.
        const LEFT_TRIGGER = 0x0000_0800;
        /// Right trigger. Has a value of `0x0000_1000`.
        const RIGHT_TRIGGER = 0x0000_1000;
        /// Left analog stick click. Has a value of `0x0000_2000`.
        const LEFT_STICK_CLICK = 0x0000_2000;
        /// Right analog stick click. Has a value of `0x0000_4000`.
        const RIGHT_STICK_CLICK = 0x0000_4000;
        /// Right analog stick left. Has a value of `0x0000_8000`.
        const RIGHT_STICK_LEFT = 0x0000_8000;
        /// Right analog stick right. Has a value of `0x0001_0000`
        const RIGHT_STICK_RIGHT = 0x0001_0000;
        /// Right analog stick up. Has a value of `0x0002_0000`
        const RIGHT_STICK_UP = 0x0002_0000;
        /// Right analog stick down. Has a value of `0x0004_0000`
        const RIGHT_STICK_DOWN = 0x0004_0000;
        /// Special button. Has a value of `0x0008_0000`.
        const SPECIAL = 0x0008_0000;
        /// UDP Action 1. Has a value of `0x0010_0000`.
        const UDP_ACTION_1 = 0x0010_0000;
        /// UDP Action 2. Has a value of `0x0020_0000`.
        const UDP_ACTION_2 = 0x0020_0000;
        /// UDP Action 3. Has a value of `0x0040_0000`.
        const UDP_ACTION_3 = 0x0040_0000;
        /// UDP Action 4. Has a value of `0x0080_0000`.
        const UDP_ACTION_4 = 0x0080_0000;
        /// UDP Action 5. Has a value of `0x0100_0000`.
        const UDP_ACTION_5 = 0x0100_0000;
        /// UDP Action 6. Has a value of `0x0200_0000`.
        const UDP_ACTION_6 = 0x0200_0000;
        /// UDP Action 7. Has a value of `0x0400_0000`.
        const UDP_ACTION_7 = 0x0400_0000;
        /// UDP Action 8. Has a value of `0x0800_0000`.
        const UDP_ACTION_8 = 0x0800_0000;
        /// UDP Action 9. Has a value of `0x1000_0000`.
        const UDP_ACTION_9 = 0x1000_0000;
        /// UDP Action 10. Has a value of `0x2000_0000`.
        const UDP_ACTION_10 = 0x2000_0000;
        /// UDP Action 11. Has a value of `0x4000_0000`.
        const UDP_ACTION_11 = 0x4000_0000;
        /// UDP Action 12. Has a value of `0x8000_0000`.
        const UDP_ACTION_12 = 0x8000_0000;
    }
}

/// Unique identifier of a driver's nationality. Represents a [`u8`].
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
    Hash,
    Serialize,
    Deserialize,
)]
#[br(little, repr(u8))]
pub enum Nationality {
    Unknown = 0,
    American = 1,
    Argentinian = 2,
    Australian = 3,
    Austrian = 4,
    Azerbaijani = 5,
    Bahraini = 6,
    Belgian = 7,
    Bolivian = 8,
    Brazilian = 9,
    British = 10,
    Bulgarian = 11,
    Cameroonian = 12,
    Canadian = 13,
    Chilean = 14,
    Chinese = 15,
    Colombian = 16,
    CostaRican = 17,
    Croatian = 18,
    Cypriot = 19,
    Czech = 20,
    Danish = 21,
    Dutch = 22,
    Ecuadorian = 23,
    English = 24,
    Emirian = 25,
    Estonian = 26,
    Finnish = 27,
    French = 28,
    German = 29,
    Ghanaian = 30,
    Greek = 31,
    Guatemalan = 32,
    Honduran = 33,
    HongKonger = 34,
    Hungarian = 35,
    Icelander = 36,
    Indian = 37,
    Indonesian = 38,
    Irish = 39,
    Israeli = 40,
    Italian = 41,
    Jamaican = 42,
    Japanese = 43,
    Jordanian = 44,
    Kuwaiti = 45,
    Latvian = 46,
    Lebanese = 47,
    Lithuanian = 48,
    Luxembourger = 49,
    Malaysian = 50,
    Maltese = 51,
    Mexican = 52,
    Monegasque = 53,
    NewZealander = 54,
    Nicaraguan = 55,
    NorthernIrish = 56,
    Norwegian = 57,
    Omani = 58,
    Pakistani = 59,
    Panamanian = 60,
    Paraguayan = 61,
    Peruvian = 62,
    Polish = 63,
    Portuguese = 64,
    Qatari = 65,
    Romanian = 66,
    Russian = 67,
    Salvadoran = 68,
    Saudi = 69,
    Scottish = 70,
    Serbian = 71,
    Singaporean = 72,
    Slovakian = 73,
    Slovenian = 74,
    SouthKorean = 75,
    SouthAfrican = 76,
    Spanish = 77,
    Swedish = 78,
    Swiss = 79,
    Thai = 80,
    Turkish = 81,
    Uruguayan = 82,
    Ukrainian = 83,
    Venezuelan = 84,
    Welsh = 85,
    Barbadian = 86,
    Vietnamese = 87,
    Algerian = 88,
    Bosnian = 89,
    Filipino = 90,
}

/// "Your telemetry" UDP setting value. Represents a [`u8`].
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
    Hash,
    Serialize,
    Deserialize,
)]
#[br(little, repr(u8))]
pub enum YourTelemetry {
    Restricted = 0,
    Public = 1,
}

/// Type of surface a tyre is on. Represents a [`u8`].
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
    Hash,
    Serialize,
    Deserialize,
)]
#[br(little, repr(u8))]
pub enum Surface {
    Tarmac = 0,
    RumbleStrip = 1,
    Concrete = 2,
    Rock = 3,
    Gravel = 4,
    Mud = 5,
    Sand = 6,
    Grass = 7,
    Water = 8,
    Cobblestone = 9,
    Metal = 10,
    Ridged = 11,
}

/// Bit flags of lit rev lights on a steering wheel.
/// Represents a [`u16`].
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Serialize, Deserialize,
)]
pub struct RevLights(u16);

bitflags! {
    impl RevLights: u16 {
        /// 1st left (1st leftmost) LED. Has a value of `0x0001`.
        const LEFT_1 = 0x0001;
        /// 2nd left (2nd leftmost) LED. Has a value of `0x0002`.
        const LEFT_2 = 0x0002;
        /// 3rd left (3rd leftmost) LED. Has a value of `0x0004`.
        const LEFT_3 = 0x0004;
        /// 4th left (4th leftmost) LED. Has a value of `0x0008`.
        const LEFT_4 = 0x0008;
        /// 5th left (5th leftmost) LED. Has a value of `0x0010`.
        const LEFT_5 = 0x0010;
        /// 1st middle (6th leftmost)  LED. Has a value of `0x0020`.
        const MIDDLE_1 = 0x0020;
        /// 2nd middle (7th leftmost) LED. Has a value of `0x0040`.
        const MIDDLE_2 = 0x0040;
        /// 3rd middle (8th leftmost) LED. Has a value of `0x0080`.
        const MIDDLE_3 = 0x0080;
        /// 4th middle (9th leftmost) LED . Has a value of `0x0100`.
        const MIDDLE_4 = 0x0100;
        /// 5th middle (10th leftmost) LED . Has a value of `0x0200`.
        const MIDDLE_5 = 0x0200;
        /// 1st right (11th leftmost) LED. Has a value of `0x0400`.
        const RIGHT_1 = 0x0400;
        /// 2nd right (12th leftmost) LED. Has a value of `0x0800`.
        const RIGHT_2 = 0x0800;
        /// 3rd right (13th leftmost) LED. Has a value of `0x1000`.
        const RIGHT_3 = 0x1000;
        /// 4th right (14th leftmost) LED. Has a value of `0x2000`.
        const RIGHT_4 = 0x2000;
        /// 5th right (15th leftmost) LED. Has a value of `0x4000`.
        const RIGHT_5 = 0x4000;
    }
}

/// Index of currently open multi-function display panel. Represents a [`u8`].
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
    Hash,
    Serialize,
    Deserialize,
)]
#[br(little, repr(u8))]
pub enum MfdPanelIndex {
    CarSetup = 0,
    Pits = 1,
    Damage = 2,
    Engine = 3,
    Temperatures = 4,
    Closed = 255,
}

/// Type of enabled traction control assist. Represents a [`u8`].
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
    Hash,
    Serialize,
    Deserialize,
)]
#[br(little, repr(u8))]
pub enum TractionControl {
    Off = 0,
    Medium = 1,
    Full = 2,
}

/// Type of fuel mix that's currently in use. Represents a [`u8`].
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
    Hash,
    Serialize,
    Deserialize,
)]
#[br(little, repr(u8))]
pub enum FuelMix {
    Lean = 0,
    Standard = 1,
    Rich = 2,
    Max = 3,
}

/// ERS deployment mode that's currently in use. Represents a [`u8`].
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
    Hash,
    Serialize,
    Deserialize,
)]
#[br(little, repr(u8))]
pub enum ErsDeployMode {
    None = 0,
    Medium = 1,
    Overtake = 2,
    Hotlap = 3,
}

/// Flag the driver is currently being shown. Represents an [`i8`].
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
    Hash,
    Serialize,
    Deserialize,
)]
#[br(little, repr(i8))]
pub enum VehicleFiaFlag {
    Unknown = -1,
    None = 0,
    Green = 1,
    Blue = 2,
    Yellow = 3,
    Red = 4,
}

/// Global DRS activation permission status. Represents an [`i8`].
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
    Hash,
    Serialize,
    Deserialize,
)]
#[br(little, repr(i8))]
pub enum DrsAllowed {
    Unknown = -1,
    NotAllowed = 0,
    Allowed = 1,
}

/// Session-independent tyre compound type. Represents a [`u8`].
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
    Hash,
    Serialize,
    Deserialize,
)]
#[br(little, repr(u8))]
pub enum ActualTyreCompound {
    Unknown = 0,
    C5 = 16,
    C4 = 17,
    C3 = 18,
    C2 = 19,
    C1 = 20,
    C0 = 21,
    Inter = 7,
    Wet = 8,
    ClassicDry = 9,
    ClassicWet = 10,
    F2SuperSoft = 11,
    F2Soft = 12,
    F2Medium = 13,
    F2Hard = 14,
    F2Wet = 15,
}

/// Visual indicator of a tyre compound's type in a given session.
/// Represents a [`u8`].
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
    Hash,
    Serialize,
    Deserialize,
)]
#[br(little, repr(u8))]
pub enum VisualTyreCompound {
    Unknown = 0,
    F1Soft = 16,
    F1Medium = 17,
    F1Hard = 18,
    F1Inter = 7,
    F1Wet = 8,
    ClassicDry = 9,
    ClassicWet = 10,
    F2SuperSoft = 19,
    F2Soft = 20,
    F2Medium = 21,
    F2Hard = 22,
    F2Wet = 15,
}

/// Readiness of a player in an online lobby. Represents a [`u8`].
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
    Hash,
    Serialize,
    Deserialize,
)]
#[br(little, repr(u8))]
pub enum ReadyStatus {
    NotReady = 0,
    Ready = 1,
    Spectating = 2,
}

/// Bit flags of lap validity across all three sectors and overall.
/// Represents a [`u8`].
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Serialize, Deserialize,
)]
pub struct LapValid(u8);

bitflags! {
    impl LapValid: u8 {
        /// Whether the whole lap is valid. Has a value of `0x01`.
        const OVERALL = 0x01;
        /// Whether the sector 1 run is valid. Has a value of `0x02`.
        const SECTOR_1 = 0x02;
        /// Whether the sector 2 run is valid. Has a value of `0x04`.
        const SECTOR_2 = 0x04;
        /// Whether the sector 3 run is valid. Has a value of `0x08`.
        const SECTOR_3 = 0x08;
    }
}

/// Speed unit used by a player. Represents a [`u8`].
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
    Hash,
    Serialize,
    Deserialize,
)]
#[br(little, repr(u8))]
pub enum SpeedUnit {
    MilesPerHour = 0,
    KilometresPerHour = 1,
}

/// Temperature unit used by a player. Represents a [`u8`].
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
    Hash,
    Serialize,
    Deserialize,
)]
#[br(little, repr(u8))]
pub enum TemperatureUnit {
    Celsius = 0,
    Fahrenheit = 1,
}

/// Console or PC game distribution platform used by a player.
/// Represents a [`u8`].
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
    Hash,
    Serialize,
    Deserialize,
)]
#[br(little, repr(u8))]
pub enum Platform {
    Invalid = 0,
    Steam = 1,
    PlayStation = 3,
    Xbox = 4,
    Origin = 6,
    Unknown = 255,
}

/// Recovery mode assist that's currently enabled.
/// Represents a [`u8`].
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
    Hash,
    Serialize,
    Deserialize,
)]
#[br(little, repr(u8))]
pub enum RecoveryMode {
    None = 0,
    Flashbacks = 1,
    AutoRecovery = 2,
}

/// Flashback usage limit that's currently enabled.
/// Represents a [`u8`].
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
    Hash,
    Serialize,
    Deserialize,
)]
#[br(little, repr(u8))]
pub enum FlashbackLimit {
    Low = 0,
    Medium = 1,
    High = 2,
    Unlimited = 3,
}

/// Type of surface simulation that's currently enabled.
/// Represents a [`u8`].
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
    Hash,
    Serialize,
    Deserialize,
)]
#[br(little, repr(u8))]
pub enum SurfaceSimType {
    Simplified = 0,
    Realistic = 1,
}

/// Difficulty of driving with low fuel. Represent a [`u8`].
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
    Hash,
    Serialize,
    Deserialize,
)]
#[br(little, repr(u8))]
pub enum LowFuelMode {
    Easy = 0,
    Hard = 1,
}

/// Race starts assist that's currently in use.
/// Represents a [`u8`].
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
    Hash,
    Serialize,
    Deserialize,
)]
#[br(little, repr(u8))]
pub enum RaceStarts {
    Manual = 0,
    Assisted = 1,
}

/// Type of tyre temperature simulation that's currently in use.
/// Represents a [`u8`].
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
    Hash,
    Serialize,
    Deserialize,
)]
#[br(little, repr(u8))]
pub enum TyreTemperature {
    SurfaceOnly = 0,
    SurfaceAndCarcass = 1,
}

/// Type of car damage simulation that's currently in use.
/// Represents a [`u8`].
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
    Hash,
    Serialize,
    Deserialize,
)]
#[br(little, repr(u8))]
pub enum CarDamage {
    Off = 0,
    Reduced = 1,
    Standard = 2,
    Simulation = 3,
}

/// Car damage severity. Represents a [`u8`].
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
    Hash,
    Serialize,
    Deserialize,
)]
#[br(little, repr(u8))]
pub enum CarDamageRate {
    Reduced = 0,
    Standard = 1,
    Simulation = 2,
}

/// Type of collision simulation that's currently enabled.
/// Represents a [`u8`].
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
    Hash,
    Serialize,
    Deserialize,
)]
#[br(little, repr(u8))]
pub enum Collisions {
    Off = 0,
    PlayerToPlayerOff = 1,
    On = 2,
}

/// Type of corner cutting and track limits punishability.
/// Represents a [`u8`].
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
    Hash,
    Serialize,
    Deserialize,
)]
#[br(little, repr(u8))]
pub enum CornerCuttingStringency {
    Regular = 0,
    Strict = 1,
}

/// The way the game handles pit stops. Represents a [`u8`].
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
    Hash,
    Serialize,
    Deserialize,
)]
#[br(little, repr(u8))]
pub enum PitStopExperience {
    Automatic = 0,
    Broadcast = 1,
    Immersive = 2,
}

/// The likelihood of safety car getting deployed with hazard on track.
/// Represents a [`u8`].
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
    Hash,
    Serialize,
    Deserialize,
)]
#[br(little, repr(u8))]
pub enum SafetyCarIntensity {
    Off = 0,
    Reduced = 1,
    Standard = 2,
    Increased = 3,
}

/// The way the game handles safety car periods. Represents a [`u8`].
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
    Hash,
    Serialize,
    Deserialize,
)]
#[br(little, repr(u8))]
pub enum SafetyCarExperience {
    Broadcast = 0,
    Immersive = 1,
    Unknown = 255,
}

/// The way the game handles formation laps. Represents a [`u8`].
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
    Hash,
    Serialize,
    Deserialize,
)]
#[br(little, repr(u8))]
pub enum FormationLapExperience {
    Broadcast = 0,
    Immersive = 1,
    Unknown = 255,
}

/// The likelihood of the game using a red flag after a serious incident.
/// Represents a [`u8`].
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
    Hash,
    Serialize,
    Deserialize,
)]
#[br(little, repr(u8))]
pub enum RedFlagIntensity {
    Off = 0,
    Reduced = 1,
    Standard = 2,
    Increased = 3,
}

/// Type of safety car being deployed in a
/// [`SafetyCar` event](variant@crate::packets::event::EventDetails::SafetyCar).
/// Represents a [`u8`].
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
    Hash,
    Serialize,
    Deserialize,
)]
#[br(little, repr(u8))]
pub enum SafetyCarType {
    None = 0,
    Full = 1,
    Virtual = 2,
    FormationLap = 3,
}

/// Type of [`SafetyCar` event](variant@crate::packets::event::EventDetails::SafetyCar).
/// Represents a [`u8`].
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
    Hash,
    Serialize,
    Deserialize,
)]
#[br(little, repr(u8))]
pub enum SafetyCarEventType {
    Deployed = 0,
    Returning = 1,
    Returned = 2,
    ResumeRace = 3,
}
