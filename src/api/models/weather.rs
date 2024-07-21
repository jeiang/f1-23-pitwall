/// Weather types for the game.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Weather {
    Clear,
    LightClouds,
    Overcast,
    LightRain,
    HeavyRain,
    Storm,
}
