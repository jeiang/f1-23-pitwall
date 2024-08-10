use num_derive::FromPrimitive;

use crate::packet::macros::generate_enum_deserialize_impls;

/// The ruleset in use for the current game session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FromPrimitive)]
pub(crate) enum RuleSet {
    PracticeAndQualifying,
    Race,
    TimeTrial,
    TimeAttack,
    CheckpointChallenge,
    Autocross,
    Drift,
    AverageSpeedZone,
    RivalDuel,
}

generate_enum_deserialize_impls!(RuleSet);
