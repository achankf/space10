use rand::{seq::SliceRandom, thread_rng, Rng};

use crate::{Faction, Game, ZoneId};

impl Game {
    pub fn get_random_settled_zone(&self) -> ZoneId {
        //TODO this is expensive; look to reduce creation of temporary vecs

        let faction_ids: Vec<_> = self
            .nations
            .iter_with_id()
            .map(|(faction_id, _)| faction_id)
            .collect();

        let faction_id = faction_ids
            .choose(&mut thread_rng())
            .expect("should have at least 1 faction");

        let zones: Vec<_> = self.nations[*faction_id]
            .owned_zones
            .iter()
            .cloned()
            .collect();

        zones
            .choose(&mut thread_rng())
            .cloned()
            .expect("faction should have at least 1 zone")
    }

    pub fn get_random_faction(&self) -> &Faction {
        let len = self.nations.len();
        let target = thread_rng().gen_range(0..len);
        self.nations
            .iter()
            .skip(target)
            .next()
            .expect("cannot choose nation")
    }
}

impl Faction {
    pub fn get_capital_zone(&self) -> ZoneId {
        self.capital_zone
    }
}
