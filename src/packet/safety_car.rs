use num_derive::FromPrimitive;

use crate::packet::macros::generate_enum_deserialize_impls;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FromPrimitive)]
pub(crate) enum Status {
    None,
    Full,
    Virtual,
    FormationLap,
}

generate_enum_deserialize_impls!(Status);
