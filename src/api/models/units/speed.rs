use num_derive::FromPrimitive;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, FromPrimitive)]
pub enum Speed {
    #[default]
    Kph,
    Mph,
}
