use num_derive::FromPrimitive;
use tokio::io::AsyncRead;

use crate::packet::{
    macros::generate_enum_deserialize_impls,
    DeserializeUDP,
    DeserializeUDPResult,
};

/// Flags for the marshal zones
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, FromPrimitive)]
pub(crate) enum ZoneFlag {
    #[default]
    Unknown = 0,
    None,
    Green,
    Blue,
    Yellow,
}

generate_enum_deserialize_impls!(ZoneFlag);

/// The marshal zones are a series of zones that cover the track. Each zone has a start point and a
/// flag. The flags are used to inform the driver of the condition of the track in that zone.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Zone {
    pub(crate) zone_start: f32,
    pub(crate) zone_flag: ZoneFlag,
}

impl DeserializeUDP for Zone {
    async fn deserialize<R>(mut reader: R) -> DeserializeUDPResult<Self>
    where
        R: AsyncRead + Unpin,
        Self: Sized,
    {
        let zone_start = f32::deserialize(&mut reader).await?;
        let zone_flag = ZoneFlag::deserialize(&mut reader).await?;
        Ok(Self { zone_start, zone_flag })
    }
}
