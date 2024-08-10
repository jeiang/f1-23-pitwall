use num_derive::FromPrimitive;

use crate::packet::macros::generate_enum_deserialize_impls;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, FromPrimitive)]
pub(crate) enum DriverStatus {
    #[default]
    InGarage = 0,
    FlyingLap,
    InLap,
    OutLap,
    OnTrack,
}

generate_enum_deserialize_impls!(DriverStatus);
