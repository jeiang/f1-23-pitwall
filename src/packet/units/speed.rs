use num_derive::FromPrimitive;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, FromPrimitive)]
pub(crate) enum Speed {
    Mph = 0,
    #[default]
    Kph,
}
