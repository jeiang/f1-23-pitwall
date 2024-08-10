use std::cmp::max;

use tokio::io::AsyncRead;
use tracing::trace;

use crate::packet::{
    deserialize_bool,
    deserialize_option,
    deserialize_vec,
    marshal,
    safety_car,
    units,
    DeserializeUDP,
    DeserializeUDPResult,
    Forecast,
    Formula,
    GameMode,
    RuleSet,
    SessionLength,
    SessionType,
    StopDetails,
    Track,
    Weather,
};

/// The player display info contains information about the player's selected units.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct PlayerUnits {
    /// The player's selected speed units
    speed_units: units::Speed,
    /// The player's selected temperature units
    temperature_units: units::Temperature,
}

impl DeserializeUDP for PlayerUnits {
    async fn deserialize<R>(mut reader: R) -> DeserializeUDPResult<Self>
    where
        R: AsyncRead + Unpin,
        Self: Sized,
    {
        let speed_units = units::Speed::deserialize(&mut reader).await?;
        let temperature_units = units::Temperature::deserialize(&mut reader).await?;
        Ok(Self {
            speed_units,
            temperature_units,
        })
    }
}

/// The session data contains all the information about the current session.
/// Frequency: 2 per seconds
/// Size: 644 bytes
/// Version: 1
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct SessionData {
    /// The current weather conditions
    weather: Weather,
    /// The track temperature in degrees Celsius
    track_temperature: i8,
    /// The air temperature in degrees Celsius
    air_temperature: i8,
    /// The total number of laps for this race
    total_laps: u8,
    /// Track length in meters
    track_length: u16,
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
    is_game_paused: bool,
    /// The car index of the car that is being spectated, if the player is spectating
    spectator_car_index: Option<u8>,
    /// Whether the player has SLI Pro support enabled
    sli_pro_native_support: bool,
    /// The marshal zones
    marshal_zones: Vec<marshal::Zone>,
    /// The safety car status
    safety_car_status: safety_car::Status,
    /// Whether the game is online or offline
    is_online: bool,
    /// The forecasted weather for the session and next sessions (if available)
    weather_forecast: Forecast,
    /// AI difficulty level 0 - 110
    ai_difficulty: u8,
    /// Season ID - persisted across saves
    season_link_id: u8,
    /// Weekend Link ID - unique identifier for the weekend
    weekend_link_id: u8,
    /// Session Link ID - unique identifier for the session
    session_link_id: u8,
    /// Pit stop strategy
    pit_stop_strategy: StopDetails,
    /// Game mode
    game_mode: GameMode,
    /// Ruleset
    ruleset: RuleSet,
    /// Time of day - minutes since midnight
    time_of_day: u32,
    /// Session length
    session_length: SessionLength,
    /// Player display info for the primary player
    lead_player: PlayerUnits,
    /// Player display info for the secondary player
    second_player: PlayerUnits,
    /// Number of safety cars in this session
    num_safety_cars: u8,
    /// Number of virtual safety cars in this session
    num_virtual_safety_cars: u8,
    /// Number of red flags in this session
    num_red_flags: u8,
}
#[allow(clippy::too_many_lines)]
impl DeserializeUDP for SessionData {
    async fn deserialize<R>(mut reader: R) -> DeserializeUDPResult<Self>
    where
        R: AsyncRead + Unpin,
        Self: Sized,
    {
        let weather = Weather::deserialize(&mut reader).await?;
        trace!("parsed weather as: {weather:?}");
        let track_temperature = i8::deserialize(&mut reader).await?;
        trace!("parsed track_temperature as {track_temperature:?}");
        let air_temperature = i8::deserialize(&mut reader).await?;
        trace!("parsed air_temperature as {air_temperature:?}");
        let total_laps = u8::deserialize(&mut reader).await?;
        trace!("parsed total_laps as {total_laps:?}");
        let track_length = u16::deserialize(&mut reader).await?;
        trace!("parsed track_length as {track_length:?}");
        let session_type = SessionType::deserialize(&mut reader).await?;
        trace!("parsed session_type as {session_type:?}");
        let track_id = Track::deserialize(&mut reader).await?;
        trace!("parsed track_id as {track_id:?}");
        let formula = Formula::deserialize(&mut reader).await?;
        trace!("parsed formula as {formula:?}");
        let session_time_left = u16::deserialize(&mut reader).await?;
        trace!("parsed session_time_left as {session_time_left:?}");
        let session_duration = u16::deserialize(&mut reader).await?;
        trace!("parsed session_duration as {session_duration:?}");
        let pit_speed_limit = u8::deserialize(&mut reader).await?;
        trace!("parsed pit_speed_limit as {pit_speed_limit:?}");
        let is_game_paused = deserialize_bool(&mut reader).await?;
        trace!("parsed is_game_paused as {is_game_paused:?}");
        let spectator_car_index = {
            let is_spectating = deserialize_bool(&mut reader).await?;
            let spectator_idx = deserialize_option(&mut reader, 255).await?;
            if is_spectating {
                spectator_idx
            } else {
                None
            }
        };
        trace!("parsed spectator_car_index as {spectator_car_index:?}");
        let sli_pro_native_support = deserialize_bool(&mut reader).await?;
        trace!("parsed sli_pro_native_support as {sli_pro_native_support:?}");
        let marshal_zones_count = max(21, u8::deserialize(&mut reader).await?);
        trace!("parsed marshal_zones_count as {marshal_zones_count:?}");
        let marshal_zones = deserialize_vec(&mut reader, marshal_zones_count as usize).await?;
        trace!("parsed marshal_zones as {marshal_zones:?}");
        let safety_car_status = safety_car::Status::deserialize(&mut reader).await?;
        trace!("parsed safety_car_status as {safety_car_status:?}");
        let is_online = deserialize_bool(&mut reader).await?;
        trace!("parsed is_online as {is_online:?}");
        let weather_forecast = Forecast::deserialize(&mut reader).await?;
        trace!("parsed weather_forecast as {weather_forecast:?}");
        let ai_difficulty = u8::deserialize(&mut reader).await?;
        trace!("parsed ai_difficulty as {ai_difficulty:?}");
        let season_link_id = u8::deserialize(&mut reader).await?;
        trace!("parsed season_link_id as {season_link_id:?}");
        let weekend_link_id = u8::deserialize(&mut reader).await?;
        trace!("parsed weekend_link_id as {weekend_link_id:?}");
        let session_link_id = u8::deserialize(&mut reader).await?;
        trace!("parsed session_link_id as {session_link_id:?}");
        let pit_stop_strategy = StopDetails::deserialize(&mut reader).await?;
        trace!("parsed pit_stop_strategy as {pit_stop_strategy:?}");
        let game_mode = GameMode::deserialize(&mut reader).await?;
        trace!("parsed game_mode as {game_mode:?}");
        let ruleset = RuleSet::deserialize(&mut reader).await?;
        trace!("parsed ruleset as {ruleset:?}");
        let time_of_day = u32::deserialize(&mut reader).await?;
        trace!("parsed time_of_day as {time_of_day:?}");
        let session_length = SessionLength::deserialize(&mut reader).await?;
        trace!("parsed session_length as {session_length:?}");
        let lead_player = PlayerUnits::deserialize(&mut reader).await?;
        trace!("parsed lead_player as {lead_player:?}");
        let second_player = PlayerUnits::deserialize(&mut reader).await?;
        trace!("parsed second_player as {second_player:?}");
        let num_safety_cars = u8::deserialize(&mut reader).await?;
        trace!("parsed num_safety_cars as {num_safety_cars:?}");
        let num_virtual_safety_cars = u8::deserialize(&mut reader).await?;
        trace!("parsed num_virtual_safety_cars as {num_virtual_safety_cars:?}");
        let num_red_flags = u8::deserialize(&mut reader).await?;
        trace!("parsed num_red_flags as {num_red_flags:?}");
        Ok(Self {
            weather,
            track_temperature,
            air_temperature,
            total_laps,
            track_length,
            session_type,
            track_id,
            formula,
            session_time_left,
            session_duration,
            pit_speed_limit,
            is_game_paused,
            spectator_car_index,
            sli_pro_native_support,
            marshal_zones,
            safety_car_status,
            is_online,
            weather_forecast,
            ai_difficulty,
            season_link_id,
            weekend_link_id,
            session_link_id,
            pit_stop_strategy,
            game_mode,
            ruleset,
            time_of_day,
            session_length,
            lead_player,
            second_player,
            num_safety_cars,
            num_virtual_safety_cars,
            num_red_flags,
        })
    }
}
