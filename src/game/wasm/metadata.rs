use crate::{Game, REGION_LENGTH, SUPER_REGION_LENGTH, ZONE_LENGTH};

impl Game {
    pub const fn get_zone_length() -> usize {
        ZONE_LENGTH
    }

    pub const fn get_region_length() -> usize {
        REGION_LENGTH
    }

    pub const fn get_super_region_length() -> usize {
        SUPER_REGION_LENGTH
    }
}
