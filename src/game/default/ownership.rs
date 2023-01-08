use crate::Ownership;

impl Default for Ownership {
    fn default() -> Self {
        Self {
            stocks: Default::default(),
            shares: Default::default(),
        }
    }
}
