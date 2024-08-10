mod speed;
mod temperature;

use crate::api::macros::generate_enum_deserialize_impls;
pub use speed::*;
pub use temperature::*;

generate_enum_deserialize_impls!(Speed, Temperature);
