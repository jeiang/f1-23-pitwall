use num_derive::FromPrimitive;

use crate::packet::macros::generate_enum_deserialize_impls;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, FromPrimitive)]
pub(crate) enum ResultStatus {
    #[default]
    Invalid = 0,
    Inactive,
    Active,
    Finished,
    DidNotFinish,
    Disqualified,
    NotClassified,
    Retired,
}

generate_enum_deserialize_impls!(ResultStatus);
