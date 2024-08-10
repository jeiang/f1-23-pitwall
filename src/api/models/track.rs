use crate::api::macros::generate_enum_deserialize_impls;
use num_derive::FromPrimitive;

/// All tracks that you can race on in F1 2023.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FromPrimitive)]
pub enum Track {
    Melbourne = 0,
    PaulRicard,
    Shanghai,
    Bahrain,
    Catalunya,
    Monaco,
    Montreal,
    Silverstone,
    Hockenheim,
    Hungaroring,
    Spa,
    Monza,
    Singapore,
    Suzuka,
    AbuDhabi,
    Texas,
    Brazil,
    Austria,
    Sochi,
    Mexico,
    Baku,
    BahrainShort,
    SilverstoneShort,
    TexasShort,
    SuzukaShort,
    Hanoi,
    Zandvoort,
    Imola,
    Portimao,
    Jeddah,
    Miami,
    LasVegas,
    Losail,
}

generate_enum_deserialize_impls!(Track);
