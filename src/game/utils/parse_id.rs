use std::convert::TryInto;

use crate::{
    CharacterId, CharacterIdSizeType, FamilyId, FamilyIdSizeType, Game, NationId, NationIdSizeType,
    RegionId, RegionIdSizeType, SuperRegionId, SuperRegionIdSizeType, TileId, TileIdSizeType,
    ZoneId, ZoneIdSizeType,
};

impl Game {
    pub fn parse_tile_id(&self, index: TileIdSizeType) -> TileId {
        assert!(index < self.surface_area());
        TileId(index)
    }

    pub fn parse_zone_id(&self, index: ZoneIdSizeType) -> ZoneId {
        assert!(index < self.total_num_zones());
        ZoneId(index)
    }

    pub fn parse_region_id(&self, index: RegionIdSizeType) -> RegionId {
        assert!(index < self.total_num_regions());
        RegionId(index)
    }

    pub fn parse_super_region_id(&self, index: SuperRegionIdSizeType) -> SuperRegionId {
        assert!(index < self.total_num_super_regions());
        SuperRegionId(index)
    }

    pub fn parse_character_id(&self, index: CharacterIdSizeType) -> Option<CharacterId> {
        let len: CharacterIdSizeType = self
            .characters
            .len()
            .try_into()
            .expect("cannot parse usize into input size");

        if index < len {
            Some(CharacterId(index))
        } else {
            None
        }
    }

    pub fn parse_faction_id(&self, index: NationIdSizeType) -> NationId {
        assert!(index < self.nations.len().try_into().unwrap());
        NationId(index)
    }

    pub fn parse_family_id(&self, index: FamilyIdSizeType) -> FamilyId {
        assert!(index < self.families.len().try_into().unwrap());
        FamilyId(index)
    }
}
