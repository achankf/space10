mod camera;
mod character;
mod default;
mod faction;
mod family;
mod game;
mod government;
mod impl_traits;
mod map;
mod resource_stockpile;
mod tile_kind;
mod utils;
mod wasm;
mod zone;

use serde::{Deserialize, Serialize};
use stable_id::{StableId, Tec};
use std::collections::{HashMap, HashSet};
use wasm_bindgen::prelude::wasm_bindgen;

const ZONE_LENGTH: usize = 5;
const REGION_LENGTH: usize = ZONE_LENGTH * ZONE_LENGTH;
const SUPER_REGION_LENGTH: usize = 4 * REGION_LENGTH;

pub type TimeType = u32;

// id types

pub type TileIdSizeType = usize;
#[derive(Debug, Serialize, Deserialize, StableId)]
pub struct TileId(TileIdSizeType);

pub type ZoneIdSizeType = u32;
#[derive(Debug, Serialize, Deserialize, StableId)]
pub struct ZoneId(ZoneIdSizeType);

pub type RegionIdSizeType = u16;
#[derive(Debug, Serialize, Deserialize, StableId)]
pub struct RegionId(RegionIdSizeType);

pub type SuperRegionIdSizeType = u8;
#[derive(Debug, Serialize, Deserialize, StableId)]
pub struct SuperRegionId(SuperRegionIdSizeType);

pub type FamilyIdSizeType = u32;
#[derive(Debug, Serialize, Deserialize, StableId)]
pub struct FamilyId(FamilyIdSizeType);

pub type CharacterIdSizeType = u32;
#[derive(Debug, Serialize, Deserialize, StableId)]
pub struct CharacterId(CharacterIdSizeType);

#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CharacterIdPair(CharacterId, CharacterId);

pub type NationIdSizeType = u16;
#[derive(Debug, Serialize, Deserialize, StableId)]
pub struct NationId(NationIdSizeType);

pub type OrganizationIdSizeType = u32;
#[derive(Debug, Serialize, Deserialize, StableId)]
pub struct OrganizationId(OrganizationIdSizeType);

pub type OrganizationKey = (OrganizationKind, OrganizationId);

pub type BuildingIdSizeType = u32;
#[derive(Debug, Serialize, Deserialize, StableId)]
pub struct BuildingId(u32);

pub type ArmyIdSizeType = u32;
#[derive(Debug, Serialize, Deserialize, StableId)]
pub struct ArmyId(ArmyIdSizeType);

pub type PartyIdSizeType = u32;
#[derive(Debug, Serialize, Deserialize, StableId)]
pub struct PartyId(PartyIdSizeType);

pub type ArtifactIdSizeType = u32;
#[derive(Debug, Serialize, Deserialize, StableId)]
pub struct ArtifactId(ArtifactIdSizeType);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Owner {
    National(NationId),
    Organization(OrganizationKey),
    Character(CharacterId),
}

pub type OwnershipIdSizeType = u32;
#[derive(Debug, Serialize, Deserialize, StableId)]
pub struct OwnershipId(OwnershipIdSizeType);

pub type OwnershipShareSizeType = u32;
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ownership {
    stocks: OwnershipShareSizeType,
    shares: HashMap<Owner, OwnershipShareSizeType>,
}

// objects
pub enum AreaKind {
    Super,
    Region,
    Zone,
    Tile,
}

const LAND_TILE_KIND: [TileKind; 5] = [
    TileKind::Desert,
    TileKind::Forest,
    TileKind::Hill,
    TileKind::Mountain,
    TileKind::Plain,
];

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
#[wasm_bindgen]
pub enum TileKind {
    #[default]
    Unknown,
    Plain,
    Desert,
    Hill,
    Mountain,
    Forest,
    Ocean,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[wasm_bindgen]
pub enum BuildingKind {
    None,

    // good distribution & basic service
    Stockpile, // start of a town/village
    Center,    // set up by founding new towns at regional centers
    Bazaar,    // extends the coverage of the center
    Village,
    RebelCamp,

    Farm,

    // private ownership
    Shack,
    Cottage,
    Mansion,
}

#[derive(Debug, Clone, Default)]
pub struct Tile {
    building: Option<(BuildingKind, BuildingId)>,
    ownership: Ownership,
}

#[derive(Clone, Default, Debug)]
pub struct Zone {
    /// each zone determines the tile type for all tiles within the zone
    tile_type: TileKind,
    is_region_center: bool,

    allegiance: Option<NationId>,
    stockpile: ResourceSpace,
}

#[derive(Clone, Debug)]
pub enum ResourceKind {
    Grain,
}

pub type ResourceKey = (ResourceKind, u8); // (type, quality)

#[derive(Clone, Default, Debug)]
pub struct ResourceSpace(HashMap<ResourceKey, u32>);

pub enum ArtifactKind {}

pub type ArtifactKey = (ArtifactKind, ArtifactId);

pub struct Inventory {
    resources: ResourceSpace,
    artifacts: HashSet<ArtifactKey>,
}

#[derive(Debug, Clone, Default)]
pub struct Region {
    center_zone_id: ZoneId,
}

pub type BuildingRef = (BuildingKind, BuildingId);

#[derive(Debug, Clone, Default, Serialize)]
pub struct Character {
    name: String,
    at: TileId,
    birth: TimeType,
    family: FamilyId,
    faction_allegiance: Option<NationId>,
}

// not planning to do family tree; we're keeping things linear
#[derive(Debug, Default, Clone)]
pub struct Family {
    name: String,
    head: Option<CharacterId>,
    kins: HashSet<CharacterId>,
}

#[derive(Debug, Default, Clone)]
pub struct Faction {
    name: String,
    owned_zones: HashSet<ZoneId>,
    color_id: u8,
    capital_zone: ZoneId,

    leader: Option<CharacterId>,
    bosses: HashMap<CharacterId, CharacterId>,
    goons: HashMap<CharacterId, HashSet<CharacterId>>,

    found_at: TimeType,
}

#[derive(Debug, Clone)]
pub struct Party {
    leader: CharacterId,
    members: HashSet<CharacterId>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum OrganizationKind {
    // decentralized
    Estate,    // owned by a count (of a region), controls a private military, offer local jobs
    Guild,     // offer cross-region economic contracts with other guildhalls within the faction
    Mercenary, // regional warrior band for military contracts

    // centralized
    Army,        // full time military of centralized factions
    Corporation, // can expand in any faction that has a stock exchange
    CentralBank, // control money supply & stock markets

    // hermit
    Sect,    // martial art - require some talent to join
    Academy, // generalist - hefty tuition or student loans
    Circle,  // wizardry - nobility
}

#[derive(Clone, Debug)]
pub struct Organization {
    r#type: OrganizationKind,
    members: HashSet<CharacterId>,
}

#[derive(Default)]
pub struct Game {
    // map
    num_columns: TileIdSizeType,
    num_rows: TileIdSizeType,
    tiles: Tec<TileId, Tile>,
    zones: Tec<ZoneId, Zone>,
    regions: Tec<RegionId, Region>,

    //// dynamic metadata
    land_super_region_set: HashSet<SuperRegionId>,
    land_super_region_vec: Vec<SuperRegionId>,
    ocean_super_region_set: HashSet<SuperRegionId>,
    ocean_super_region_vec: Vec<SuperRegionId>,

    nations: Tec<NationId, Faction>,

    character_locations: HashMap<TileId, HashSet<CharacterId>>,
    characters: Tec<CharacterId, Character>,
    families: Tec<FamilyId, Family>,

    settled_regions: HashSet<RegionId>,

    organizations: Tec<OrganizationId, Organization>,
    parties: Tec<PartyId, Party>,

    pub player: Option<CharacterId>,

    time: TimeType,
}
