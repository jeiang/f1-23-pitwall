use num_derive::FromPrimitive;

use crate::packet::macros::generate_enum_deserialize_impls;

/// The ruleset in use for the current game session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FromPrimitive)]
pub(crate) enum RuleSet {
    PracticeAndQualifying = 0,
    Race = 1,
    TimeTrial = 2,
    TimeAttack = 4,
    CheckpointChallenge = 6,
    Autocross = 8,
    Drift = 9,
    AverageSpeedZone = 10,
    RivalDuel = 11,
}

generate_enum_deserialize_impls!(RuleSet);
