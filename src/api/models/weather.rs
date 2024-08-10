use crate::api::{
    macros::generate_enum_deserialize_impls,
    models::SessionType,
    packet::{
        deserialize_vec,
        DeserializeUDP,
        DeserializeUDPResult,
    },
};
use num_derive::FromPrimitive;
use std::cmp::max;
use tokio::io::AsyncRead;

/// Weather types for the game.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, FromPrimitive)]
pub enum Weather {
    #[default]
    Clear = 0,
    LightClouds,
    Overcast,
    LightRain,
    HeavyRain,
    Storm,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, FromPrimitive)]
pub enum ForecastAccuracy {
    #[default]
    Perfect = 0,
    Approximate,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, FromPrimitive)]
pub enum TemperatureChange {
    Increasing,
    Decreasing,
    #[default]
    None,
}

generate_enum_deserialize_impls!(Weather, ForecastAccuracy, TemperatureChange);

#[derive(Debug, Clone, PartialEq)]
pub struct Forecast {
    /// Weather forecast samples
    weather_forecast_samples: Vec<ForecastSample>,
    /// How accurate the weather forecast is
    forecast_accuracy: ForecastAccuracy,
}

impl DeserializeUDP for Forecast {
    async fn deserialize<R>(mut reader: R) -> DeserializeUDPResult<Self>
    where
        R: AsyncRead + Unpin,
        Self: Sized,
    {
        let count = max(55, u8::deserialize(&mut reader).await?);
        let weather_forecast_samples = deserialize_vec(&mut reader, count as usize).await?;
        let forecast_accuracy = ForecastAccuracy::deserialize(&mut reader).await?;
        Ok(Self {
            weather_forecast_samples,
            forecast_accuracy,
        })
    }
}

/// The weather forecast sample contains the weather forecast for a specific time offset.
#[derive(Debug, Clone, PartialEq)]
pub struct ForecastSample {
    /// The session type for this forecast
    pub session_type: SessionType,
    /// The time offset for this forecast
    pub time_offset: u8,
    /// The weather conditions for this forecast
    pub weather: Weather,
    /// The track temperature in degrees Celsius and whether it is increasing or decreasing
    pub track_temperature: (i8, TemperatureChange),
    /// The air temperature in degrees Celsius and whether it is increasing or decreasing
    pub air_temperature: (i8, TemperatureChange),
    /// The chance of rain as a percentage (0-100)
    pub rain_percentage: u8,
}

impl DeserializeUDP for ForecastSample {
    async fn deserialize<R>(mut reader: R) -> DeserializeUDPResult<Self>
    where
        R: AsyncRead + Unpin,
        Self: Sized,
    {
        let session_type = SessionType::deserialize(&mut reader).await?;
        let time_offset = u8::deserialize(&mut reader).await?;
        let weather = Weather::deserialize(&mut reader).await?;
        let track_temperature = (
            i8::deserialize(&mut reader).await?,
            TemperatureChange::deserialize(&mut reader).await?,
        );
        let air_temperature = (
            i8::deserialize(&mut reader).await?,
            TemperatureChange::deserialize(&mut reader).await?,
        );
        let rain_percentage = u8::deserialize(&mut reader).await?;

        Ok(Self {
            session_type,
            time_offset,
            weather,
            track_temperature,
            air_temperature,
            rain_percentage,
        })
    }
}
