use num_derive::FromPrimitive;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, FromPrimitive)]
pub(crate) enum Temperature {
    #[default]
    Celsius,
    Fahrenheit,
}
