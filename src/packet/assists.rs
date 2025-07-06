use num_derive::FromPrimitive;
use tokio::io::{AsyncRead, AsyncReadExt};

use crate::packet::macros::generate_enum_deserialize_impls;
use crate::packet::{DeserializeUDP, DeserializeUDPResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FromPrimitive)]
pub(crate) enum BrakingAssistLevel {
    Off = 0,
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FromPrimitive)]
pub(crate) enum GearAssist {
    Manual = 1,
    Automatic = 2,
    ManualWithSuggestedGear = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FromPrimitive)]
pub(crate) enum RacingLineAssist {
    Full,
    CornersOnly,
    Off,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FromPrimitive)]
pub(crate) enum DynamicRacingLineType {
    Line2D,
    Line3D,
}

generate_enum_deserialize_impls!(
    BrakingAssistLevel,
    GearAssist,
    RacingLineAssist,
    DynamicRacingLineType
);

#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Assists {
    /// Whether the player has steering assist enabled
    steering: bool,
    /// The level of braking assist the player has enabled
    braking: BrakingAssistLevel,
    /// Whether the player uses manual or automatic gears
    gear: GearAssist,
    /// Whether the player has pit assist enabled
    pit: bool,
    /// Whether the player has pit release assist enabled
    pit_release: bool,
    /// Whether the player has ERS assist enabled
    ers: bool,
    /// Whether the player has DRS assist enabled
    drs: bool,
    /// Whether the player has dynamic racing line enabled
    racing_line: (RacingLineAssist, DynamicRacingLineType),
}

impl DeserializeUDP for Assists {
    async fn deserialize<R>(mut reader: R) -> DeserializeUDPResult<Self>
    where
        R: AsyncRead + Unpin,
        Self: Sized,
    {
        let steering = reader.read_u8().await? == 1;
        let braking = BrakingAssistLevel::deserialize(&mut reader).await?;
        let gear = GearAssist::deserialize(&mut reader).await?;
        let pit = reader.read_u8().await? == 1;
        let pit_release = reader.read_u8().await? == 1;
        let ers = reader.read_u8().await? == 1;
        let drs = reader.read_u8().await? == 1;
        let racing_line_assist = RacingLineAssist::deserialize(&mut reader).await?;
        let dynamic_racing_line_type = DynamicRacingLineType::deserialize(&mut reader).await?;
        Ok(Self {
            steering,
            braking,
            gear,
            pit,
            pit_release,
            ers,
            drs,
            racing_line: (racing_line_assist, dynamic_racing_line_type),
        })
    }
}
