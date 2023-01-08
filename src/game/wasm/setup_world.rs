use crate::{
    utils::{faction_name_generator::generate_faction_name, js::log},
    BuildingId, Family, Game, NationId, RegionId, SuperRegionId, SuperRegionIdSizeType,
    {BuildingKind, Faction, TileKind, LAND_TILE_KIND},
};
use rand::distributions::WeightedIndex;
use rand::{seq::SliceRandom, thread_rng, Rng};
use rand_distr::Distribution;
use stable_id::{CastUsize, Inner};
use std::collections::HashSet;

impl Game {
    fn randomly_assign_land_tiles(&mut self) {
        fn to_base_weight(land_tile_type: TileKind) -> u8 {
            match land_tile_type {
                TileKind::Desert => 20,
                TileKind::Forest => 30,
                TileKind::Hill => 10,
                TileKind::Mountain => 10,
                TileKind::Plain => 30,
                _ => unreachable!("trying to map non-land tile to weight"),
            }
        }

        let base_dist = WeightedIndex::new(
            LAND_TILE_KIND
                .iter()
                .map(|land_type| to_base_weight(*land_type)),
        )
        .expect("cannot setup weighted index");

        for super_region_id in &self.land_super_region_vec {
            fn to_weight(land_tile_type: TileKind, majority_type: TileKind) -> u8 {
                let base = to_base_weight(land_tile_type);

                if land_tile_type != majority_type {
                    base
                } else {
                    base + 50
                }
            }

            let majority_type = LAND_TILE_KIND[base_dist.sample(&mut thread_rng())];
            let weights = LAND_TILE_KIND
                .iter()
                .map(|land_type| to_weight(*land_type, majority_type));
            let dist = WeightedIndex::new(weights).expect("cannot setup weighted index");

            let land_zones = self.subdivide_square_areas_super_to_zone(*super_region_id);

            for zone_id in land_zones {
                let chosen_tile_type = LAND_TILE_KIND[dist.sample(&mut thread_rng())];

                let zone = &mut self.zones[zone_id];
                assert!(zone.tile_type == TileKind::Unknown);
                zone.tile_type = chosen_tile_type;
            }
        }
    }

    fn assign_ocean_tiles(&mut self) {
        for super_region_id in &self.ocean_super_region_vec {
            let ocean_zones = self.subdivide_square_areas_super_to_zone(*super_region_id);

            for zone_id in ocean_zones {
                let zone = &mut self.zones[zone_id];
                assert!(zone.tile_type == TileKind::Unknown);
                zone.tile_type = TileKind::Ocean;
            }
        }
    }

    fn assign_centers(&mut self) {
        for region_id in 0..self.total_num_regions() {
            let region_id = self.parse_region_id(region_id);
            let zone_id = self.get_random_zone_id_in_region(region_id);

            assert!(zone_id.project() < self.total_num_zones());

            self.zones[zone_id].is_region_center = true;
            self.regions[region_id].center_zone_id = zone_id;
        }
    }

    fn alloc_factions(&mut self) {
        let num_factions = self.total_num_land_regions() / 6;
        let mut unique_faction_names = HashSet::with_capacity(num_factions.into());

        // create factions
        for _ in 0..num_factions {
            let name = loop {
                let name = generate_faction_name();
                if unique_faction_names.insert(name.clone()) {
                    break name;
                }
            };

            let faction = Faction {
                name,
                color_id: thread_rng().gen(),
                found_at: self.time,
                ..Default::default()
            };

            self.nations.alloc(faction);
        }
    }

    // helper for `spawn_towns_and_people()`. For some reason rustfmt doesn't format this unless I separate the helper.
    fn spawn_for_one_faction(
        &mut self,
        nation_id: NationId,
        visited_regions: &mut HashSet<RegionId>,
    ) {
        let center_zone_id = loop {
            let region_id = self.get_random_land_region_id();
            let region = &self[region_id];
            let center_zone_id = region.center_zone_id;
            let zone = &self[region.center_zone_id];
            if !zone.is_settled() && visited_regions.insert(region_id) {
                break center_zone_id;
            }
        };

        let mut center_zone = &mut self[center_zone_id];
        center_zone.allegiance = Some(nation_id);

        let faction = &mut self.nations[nation_id];
        faction.owned_zones.insert(center_zone_id);
        faction.capital_zone = center_zone_id;

        let tile_id = self.to_zone_center_id(center_zone_id);
        self.tiles[tile_id].building = Some((BuildingKind::Center, BuildingId(0)));

        let num_people = thread_rng().gen_range(20..30);
        for people_count in 0..num_people {
            let family_id = self.families.alloc(Family {
                name: (format!("Last {people_count}")),
                ..Default::default()
            });

            let character_id =
                self.spawn_character("Bob".to_string(), tile_id, family_id, Some(nation_id));

            self.families[family_id].kins.insert(character_id);
            self.families[family_id].head = Some(character_id);
        }

        let nation = &mut self.nations[nation_id];
        let leader_id = *self
            .character_locations
            .get(&tile_id)
            .and_then(|characters| {
                let len = characters.len();
                let index = thread_rng().gen_range(0..len - 1);
                characters.iter().nth(index)
            })
            .expect("should have at least 1 character in the tile to appoint as the monarch");

        nation.leader = Some(leader_id);
    }

    fn spawn_towns_and_people(&mut self) {
        // create starting towns per faction
        let mut visited_regions: HashSet<_> = Default::default();
        let len = self.nations.len();
        let spawn_for_one_faction =
            |faction_id| self.spawn_for_one_faction(faction_id, &mut visited_regions);

        (0..len)
            .map(CastUsize::cast_from)
            .for_each(spawn_for_one_faction)
    }

    fn split_land_and_ocean(&mut self) {
        let num_super_regions = self.total_num_super_regions();

        let super_region_ids_shuffled = {
            let mut super_region_ids: Vec<_> = (0..num_super_regions).map(SuperRegionId).collect();
            super_region_ids.shuffle(&mut thread_rng());
            super_region_ids
        };

        let num_ocean_super_regions = (num_super_regions as f64 * 0.7) as SuperRegionIdSizeType;

        log(format!(
            "subdividing super regions: ocean={}, land={}",
            num_ocean_super_regions,
            num_super_regions - num_ocean_super_regions
        ));

        let (ocean_super_regions, land_super_regions) =
            super_region_ids_shuffled.split_at(num_ocean_super_regions.into());
        self.land_super_region_set = land_super_regions.iter().cloned().collect();
        self.land_super_region_vec = land_super_regions.to_vec();
        self.ocean_super_region_set = ocean_super_regions.iter().cloned().collect();
        self.ocean_super_region_vec = ocean_super_regions.to_vec();
    }

    pub fn setup_world(&mut self) {
        self.split_land_and_ocean();
        self.randomly_assign_land_tiles();
        self.assign_ocean_tiles();

        assert!(
            self.zones
                .iter()
                .any(|zone| zone.tile_type != TileKind::Unknown),
            "there's a gap in the math"
        );

        self.assign_centers();
        self.alloc_factions();
        self.spawn_towns_and_people();
    }
}
