use num_derive::FromPrimitive;

use crate::packet::macros::generate_enum_deserialize_impls;

/// List of all race modes in the game.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FromPrimitive)]
pub(crate) enum Formula {
    F1Modern,
    F1Classic,
    F2,
    F1Generic,
    Beta,
    Supercar,
}

generate_enum_deserialize_impls!(Formula);
