mod speed;
mod temperature;

pub(crate) use speed::*;
pub(crate) use temperature::*;

use crate::packet::macros::generate_enum_deserialize_impls;

generate_enum_deserialize_impls!(Speed, Temperature);
