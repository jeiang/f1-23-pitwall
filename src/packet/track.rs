use num_derive::FromPrimitive;

use crate::packet::macros::generate_enum_deserialize_impls;

/// All tracks that you can race on in F1 2023.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FromPrimitive)]
pub(crate) enum Track {
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
