use num_derive::FromPrimitive;

use crate::packet::macros::generate_enum_deserialize_impls;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FromPrimitive)]
pub(crate) enum SessionLength {
    None,
    VeryShort,
    Short,
    Medium,
    MediumLong,
    Long,
    Full,
}

generate_enum_deserialize_impls!(SessionLength);
