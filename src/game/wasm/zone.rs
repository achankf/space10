use stable_id_traits::Inner;

use crate::{Game, RegionIdSizeType, ZoneIdSizeType};

impl Game {
    pub fn w_get_region_center_zone_id(&self, region_id: RegionIdSizeType) -> ZoneIdSizeType {
        let region_id = self.parse_region_id(region_id);
        self.get_region_center_zone_id(region_id).project()
    }
}
