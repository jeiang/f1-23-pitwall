use num_derive::FromPrimitive;
use tokio::io::AsyncRead;

use crate::packet::{
    macros::generate_enum_deserialize_impls,
    DeserializeUDP,
    DeserializeUDPResult,
};

/// Pit Status of a car

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, FromPrimitive)]
pub(crate) enum PitStatus {
    #[default]
    None,
    Pitting,
    InPitArea,
}

generate_enum_deserialize_impls!(PitStatus);

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct StopDetails {
    /// Ideal lap to pit on for this strategy
    ideal_lap: u8,
    /// Latest lap to pit on for this strategy
    latest_lap: u8,
    /// The position where you will rejoin the track
    estimated_position: u8,
}

impl DeserializeUDP for StopDetails {
    async fn deserialize<R>(mut reader: R) -> DeserializeUDPResult<Self>
    where
        R: AsyncRead + Unpin,
        Self: Sized,
    {
        let ideal_lap = u8::deserialize(&mut reader).await?;
        let latest_lap = u8::deserialize(&mut reader).await?;
        let estimated_position = u8::deserialize(&mut reader).await?;
        Ok(Self {
            ideal_lap,
            latest_lap,
            estimated_position,
        })
    }
}
