use crate::api::macros::generate_enum_deserialize_impls;
use num_derive::FromPrimitive;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FromPrimitive)]
pub enum SessionLength {
    None,
    VeryShort,
    Short,
    Medium,
    MediumLong,
    Long,
    Full,
}

generate_enum_deserialize_impls!(SessionLength);
