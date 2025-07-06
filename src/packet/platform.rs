use num_derive::FromPrimitive;

use crate::packet::macros::generate_enum_deserialize_impls;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, FromPrimitive)]
pub(crate) enum Platform {
    #[default]
    Steam = 1,
    Playstation = 3,
    Xbox = 4,
    Origin = 6,
    Unknown = 255,
}

generate_enum_deserialize_impls!(Platform);
