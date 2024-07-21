use crate::api::models::{
    formula::Formula,
    session_type::SessionType,
    track::Track,
    weather::Weather,
    ChangeDirection,
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
    /// The track temperature in degrees celsius and whether it is increasing or decreasing
    pub track_temperature: (i8, ChangeDirection),
    /// The air temperature in degrees celsius and whether it is increasing or decreasing
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

/// The session data contains all the information about the current session.
/// Frequency: 2 per seconds
/// Size: 149 bytes
/// Version: 1
#[derive(Debug)]
pub struct SessionData {
    /// The current weather conditions
    weather: Weather,
    /// The track temperature in degrees celsius
    track_temperature: i8,
    /// The air temperature in degrees celsius
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
}
