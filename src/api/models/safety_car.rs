use crate::api::macros::generate_enum_deserialize_impls;
use num_derive::FromPrimitive;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FromPrimitive)]
pub enum Status {
    None,
    Full,
    Virtual,
    FormationLap,
}

generate_enum_deserialize_impls!(Status);
