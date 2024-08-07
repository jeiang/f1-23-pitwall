pub mod driver;
pub mod formula;
pub mod game_mode;
pub mod ruleset;
pub mod session_type;
pub mod team;
pub mod track;
pub mod weather;

/// Generic struct for the F1 Version number
#[derive(Debug, Default)]
pub struct Version {
    pub major: u8,
    pub minor: u8,
}

/// Generic struct for the F1 Vector3
#[derive(Debug, Default)]
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

/// Generic struct representing a change in direction
/// Used in the `crate::packet::session_data::WeatherForecastSample` for the temperature fields
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChangeDirection {
    Up,
    Down,
    #[default]
    None,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Availability {
    Enabled,
    #[default]
    Disabled,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpeedUnits {
    #[default]
    Kph,
    Mph,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TemperatureUnits {
    #[default]
    Celsius,
    Fahrenheit,
}
