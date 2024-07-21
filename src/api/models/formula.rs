/// List of all race modes in the game.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Formula {
    F1Modern,
    F1Classic,
    F2,
    F1Generic,
    Beta,
    Supercar,
}
