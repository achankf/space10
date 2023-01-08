use crate::{Game, NationIdSizeType, ZoneIdSizeType};
use stable_id_traits::Inner;

impl Game {
    pub fn w_get_factions_len(&self) -> NationIdSizeType {
        self.nations.len() as NationIdSizeType
    }

    pub fn w_get_faction_name(&self, faction_id: NationIdSizeType) -> String {
        let faction_id = self.parse_faction_id(faction_id);
        self.nations[faction_id].name.clone()
    }

    pub fn w_get_faction_num_residents(&self, _faction_id: NationIdSizeType) -> usize {
        0
    }

    pub fn w_get_faction_owned_zones(&self, faction_id: NationIdSizeType) -> Vec<ZoneIdSizeType> {
        let faction_id = self.parse_faction_id(faction_id);
        self.nations[faction_id]
            .owned_zones
            .iter()
            .cloned()
            .map(Inner::project)
            .collect()
    }

    pub fn w_get_faction_color(&self, faction_id: NationIdSizeType) -> String {
        let faction_id = self.parse_faction_id(faction_id);
        let color_id = self.nations[faction_id].color_id;
        Self::get_color(color_id, None)
    }

    pub fn w_get_zone_allegiance(&self, zone_id: ZoneIdSizeType) -> Option<NationIdSizeType> {
        let zone_id = self.parse_zone_id(zone_id);
        self.zones[zone_id].allegiance.map(Inner::project)
    }

    pub fn w_get_capital_zone(&self, faction_id: NationIdSizeType) -> ZoneIdSizeType {
        let faction_id = self.parse_faction_id(faction_id);
        self.nations[faction_id].capital_zone.project()
    }
}
