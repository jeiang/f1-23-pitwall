use crate::api::macros::generate_enum_deserialize_impls;
use num_derive::FromPrimitive;

// TODO: determine which code is for a sprint race, and what R3 is
/// The type of session that is currently running
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, FromPrimitive)]
pub enum SessionType {
    #[default]
    Unknown = 0,
    Practice1,
    Practice2,
    Practice3,
    ShortPractice,
    Qualifying1,
    Qualifying2,
    Qualifying3,
    ShortQualifying,
    OneShotQualifying,
    Race,
    Race2,
    Race3,
    TimeTrial,
}

generate_enum_deserialize_impls!(SessionType);
