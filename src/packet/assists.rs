use num_derive::FromPrimitive;

use crate::packet::macros::generate_enum_deserialize_impls;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FromPrimitive)]
pub(crate) enum BrakingAssistLevel {
    Off = 0,
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FromPrimitive)]
pub(crate) enum GearAssist {
    Manual,
    Automatic,
    ManualWithSuggestedGear,
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

generate_enum_deserialize_impls!(BrakingAssistLevel, GearAssist, RacingLineAssist, DynamicRacingLineType);

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
