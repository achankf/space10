use crate::Zone;

impl Zone {
    pub fn is_settled(&self) -> bool {
        self.allegiance.is_some()
    }
}
