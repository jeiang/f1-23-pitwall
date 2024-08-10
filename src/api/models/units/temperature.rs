use num_derive::FromPrimitive;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, FromPrimitive)]
pub enum Temperature {
    #[default]
    Celsius,
    Fahrenheit,
}
