use crate::api::macros::generate_enum_deserialize_impls;
use num_derive::FromPrimitive;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, FromPrimitive)]
pub enum Sector {
    #[default]
    Sector1,
    Sector2,
    Sector3,
}

generate_enum_deserialize_impls!(Sector);
