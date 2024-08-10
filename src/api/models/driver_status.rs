use crate::api::macros::generate_enum_deserialize_impls;
use num_derive::FromPrimitive;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, FromPrimitive)]
pub enum DriverStatus {
    #[default]
    InGarage = 0,
    FlyingLap,
    InLap,
    OutLap,
    OnTrack,
}

generate_enum_deserialize_impls!(DriverStatus);
