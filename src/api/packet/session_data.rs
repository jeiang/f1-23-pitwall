use crate::api::models::{
    formula::Formula,
    game_mode::GameMode,
    ruleset::RuleSet,
    session_type::SessionType,
    track::Track,
    weather::Weather,
    Availability,
    ChangeDirection,
    SpeedUnits,
    TemperatureUnits,
};

/// Flags for the marshal zones
#[derive(Debug)]
pub enum MarshalZoneFlag {
    Unknown,
    None,
    Green,
    Blue,
    Yellow,
}

/// The marshal zones are a series of zones that cover the track. Each zone has a start point and a
/// flag. The flags are used to inform the driver of the condition of the track in that zone.
#[derive(Debug)]
pub struct MarshalZone {
    pub zone_start: f32,
    pub zone_flag: MarshalZoneFlag,
}

/// The weather forecast sample contains the weather forecast for a specific time offset.
#[derive(Debug)]
pub struct WeatherForecastSample {
    /// The session type for this forecast
    pub session_type: SessionType,
    /// The time offset for this forecast
    pub time_offset: u8,
    /// The weather conditions for this forecast
    pub weather: Weather,
    /// The track temperature in degrees Celsius and whether it is increasing or decreasing
    pub track_temperature: (i8, ChangeDirection),
    /// The air temperature in degrees Celsius and whether it is increasing or decreasing
    pub air_temperature: (i8, ChangeDirection),
    /// The chance of rain as a percentage (0-100)
    pub rain_percentage: u8,
}

#[derive(Debug)]
pub enum SafetyCarStatus {
    None,
    Full,
    Virtual,
    FormationLap,
}

#[derive(Debug)]
pub struct PitStopInfo {
    /// Ideal lap to pit on for this strategy
    ideal_lap: u8,
    /// Latest lap to pit on for this strategy
    latest_lap: u8,
    /// The position where you will rejoin the track
    estimated_position: u8,
}

#[derive(Debug)]
pub enum NetworkGame {
    Offline,
    Online,
}

#[derive(Debug)]
pub enum WeatherForecastAccuracy {
    Perfect,
    Approximate,
}

#[derive(Debug)]
pub enum BrakingAssistLevel {
    Off,
    Low,
    Medium,
    High,
}

#[derive(Debug)]
pub enum GearAssist {
    Manual,
    Automatic,
    ManualWithSuggestedGear,
}

#[derive(Debug)]
pub enum RacingLineAssist {
    Full,
    CornersOnly,
    Off,
}

#[derive(Debug)]
pub enum DynamicRacingLineType {
    Line2D,
    Line3D,
}

#[derive(Debug)]
pub enum SessionLength {
    None,
    VeryShort,
    Short,
    Medium,
    MediumLong,
    Long,
    Full,
}

/// The player display info contains information about the player's selected units.
#[derive(Debug)]
pub struct PlayerDisplayInfo {
    /// The player's selected speed units
    speed_units: SpeedUnits,
    /// The player's selected temperature units
    temperature_units: TemperatureUnits,
}

#[derive(Debug)]
pub struct Assists {
    /// Whether the player has steering assist enabled
    steering: Availability,
    /// The level of braking assist the player has enabled
    braking: BrakingAssistLevel,
    /// Whether the player uses manual or automatic gears
    gear: GearAssist,
    /// Whether the player has pit assist enabled
    pit: Availability,
    /// Whether the player has pit release assist enabled
    pit_release: Availability,
    /// Whether the player has ERS assist enabled
    ers: Availability,
    /// Whether the player has DRS assist enabled
    drs: Availability,
    /// Whether the player has dynamic racing line enabled
    racing_line: (RacingLineAssist, DynamicRacingLineType),
}

/// The session data contains all the information about the current session.
/// Frequency: 2 per seconds
/// Size: 644 bytes
/// Version: 1
#[derive(Debug)]
pub struct SessionData {
    /// The current weather conditions
    weather: Weather,
    /// The track temperature in degrees Celsius
    track_temperature: i8,
    /// The air temperature in degrees Celsius
    air_temperature: i8,
    /// The total number of laps for this race
    total_laps: u8,
    /// Track length in meters
    track_length: f32,
    /// The session type
    session_type: SessionType,
    /// The track that is being raced on
    track_id: Track,
    /// Type of Formula series
    formula: Formula,
    /// Time left in the session in seconds
    session_time_left: u16,
    /// The session duration in seconds
    session_duration: u16,
    /// The pit speed limit in kilometers per hour
    pit_speed_limit: u8,
    /// Whether the game is paused
    game_paused: bool,
    /// The car index of the car that is being spectated, if the player is spectating
    spectator_car_index: Option<u8>,
    /// Whether the player has SLI Pro support enabled
    sli_pro_native_support: bool,
    /// The marshal zones
    marshal_zones: Vec<MarshalZone>,
    /// The safety car status
    safety_car_status: SafetyCarStatus,
    /// Whether the game is online or offline
    network_game: NetworkGame,
    /// Weather forecast samples
    weather_forecast_samples: Vec<WeatherForecastSample>,
    /// How accurate the weather forecast is
    forecast_accuracy: WeatherForecastAccuracy,
    /// AI difficulty level 0 - 110
    ai_difficulty: u8,
    /// Season ID - persisted across saves
    season_link_id: u8,
    /// Weekend Link ID - unique identifier for the weekend
    weekend_link_id: u8,
    /// Session Link ID - unique identifier for the session
    session_link_id: u8,
    /// Pit stop strategy
    pit_stop_strategy: PitStopInfo,
    /// Game mode
    game_mode: GameMode,
    /// Ruleset
    ruleset: RuleSet,
    /// Time of day - minutes since midnight
    time_of_day: u32,
    /// Session length
    session_length: SessionLength,
    /// Player display info for the primary player
    lead_player: PlayerDisplayInfo,
    /// Player display info for the secondary player
    second_player: PlayerDisplayInfo,
    /// Number of safety cars in this session
    num_safety_cars: u8,
    /// Number of virtual safety cars in this session
    num_virtual_safety_cars: u8,
    /// Number of red flags in this session
    num_red_flags: u8,
}
