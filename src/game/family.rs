use crate::{CharacterId, FamilyId, Game};

impl Game {
    pub fn get_family_name(&self, family_id: FamilyId) -> String {
        self.families[family_id].name.clone()
    }

    pub fn get_family_head(&self, family_id: FamilyId) -> Option<CharacterId> {
        self.families[family_id].head.clone()
    }

    pub fn get_family_kins(&self, family_id: FamilyId) -> Vec<CharacterId> {
        self.families[family_id].kins.iter().cloned().collect()
    }

    pub fn get_family_kin_len(&self, family_id: FamilyId) -> usize {
        self.families[family_id].kins.len()
    }
}
