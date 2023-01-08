use crate::Game;

impl PartialEq for Game {
    /// - assumes equality when the `time` is the same
    /// - excepting a facade structure (i.e. GameContext) to keep track of a "dirty" flag
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time
    }
}
