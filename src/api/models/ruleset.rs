use crate::api::macros::generate_enum_deserialize_impls;
use num_derive::FromPrimitive;

/// The ruleset in use for the current game session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FromPrimitive)]
pub enum RuleSet {
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
