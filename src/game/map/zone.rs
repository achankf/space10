use crate::{Game, ZoneId};

impl Game {
    pub fn is_zone_settled(&self, zone_id: ZoneId) -> bool {
        self.zones[zone_id].allegiance.is_some()
    }
}
