use crate::api::macros::generate_enum_deserialize_impls;
use num_derive::FromPrimitive;

/// List of all race modes in the game.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FromPrimitive)]
pub enum Formula {
    F1Modern,
    F1Classic,
    F2,
    F1Generic,
    Beta,
    Supercar,
}

generate_enum_deserialize_impls!(Formula);
