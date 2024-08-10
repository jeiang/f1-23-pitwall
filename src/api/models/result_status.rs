use crate::api::macros::generate_enum_deserialize_impls;
use num_derive::FromPrimitive;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, FromPrimitive)]
pub enum ResultStatus {
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
