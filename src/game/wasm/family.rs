use stable_id_traits::Inner;

use crate::{CharacterIdSizeType, FamilyIdSizeType, Game};

impl Game {
    pub fn w_get_family_name(&self, family_id: FamilyIdSizeType) -> String {
        let family_id = self.parse_family_id(family_id);
        self.families[family_id].name.clone()
    }

    pub fn w_get_family_head(&self, family_id: FamilyIdSizeType) -> Option<CharacterIdSizeType> {
        let family_id = self.parse_family_id(family_id);
        self.get_family_head(family_id).map(Inner::project)
    }

    pub fn w_get_family_kins(&self, family_id: FamilyIdSizeType) -> Vec<CharacterIdSizeType> {
        let family_id = self.parse_family_id(family_id);
        self.get_family_kins(family_id)
            .into_iter()
            .map(Inner::project)
            .collect()
    }

    pub fn w_get_family_kin_len(&self, family_id: FamilyIdSizeType) -> usize {
        let family_id = self.parse_family_id(family_id);
        self.families[family_id].kins.len()
    }

    pub fn w_get_family_len(&self) -> usize {
        self.families.len()
    }
}
