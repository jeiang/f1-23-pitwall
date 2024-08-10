use num_derive::FromPrimitive;

use crate::packet::macros::generate_enum_deserialize_impls;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, FromPrimitive)]
pub(crate) enum Sector {
    #[default]
    Sector1,
    Sector2,
    Sector3,
}

generate_enum_deserialize_impls!(Sector);
