use crate::base::{Cost, Age, Unit as BaseUnit};
use crate::resource::{Id as RId, Map as RMap};
use crate::effect::List as Effects;
use std::collections::HashMap;
use lazy_static::lazy_static;

pub type List = Vec<Id>;

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub enum Id {
    LumberYard = 100,
    LoggingCamp,
    ClayPool,
    ClayPit,
    Quarry,
    StonePit,
    GlassWorks,
    Press,
    GuardTower,
    Workshop,
    Apothecary,
    StoneReserve,
    ClayReserve,
    WoodReserve,
    Stable,
    Garrison,
    Palisade,
    Scriptorium,
    Pharmacist,
    Theater,
    Altar,
    Baths,
    Tavern,

    SawMill = 200,
    BrickYard,
    ShelfQuarry,
    GlassBlower,
    DryingRoom,
    Walls,
    Forum,
    Caravansery,
    CustomHouse,
    CourtHouse,
    HorseBreeders,
    Barracks,
    ArcheryRange,
    ParadeGround,
    Library,
    Dispensary,
    School,
    Laboratory,
    Statue,
    Temple,
    Aqueduct,
    Rostrum,
    Brewery,

    Arsenal = 300,
    Pretorium,
    Academy,
    Study,
    ChamberOfCommerce,
    Port,
    Armory,
    Palace,
    TownHall,
    Obelisk,
    Fortifications,
    SiegeWorkshop,
    Circus,
    University,
    Observatory,
    Gardens,
    Pantheon,
    Senate,
    Lighthouse,
    Arena,

    MerchantsGuild = 400,
    ShipOwnersGuild,
    BuildersGuild,
    MagistratesGuild,
    ScientistsGuild,
    MoneyLendersGuild,
    TacticiansGuild,
}

#[derive(Debug)]
pub enum Group {
    RawMaterials = 1,
    ManufacturedGoods,
    Military,
    Scientific,
    Civilian,
    Commercial,
    Guild,
}

pub struct Unit {
    pub id: Id,
    pub age: Age,
    pub group: Group,
    pub cost: Cost,
    pub effects: Effects,
}

impl BaseUnit for Unit {
    fn effects(&self) -> &Effects {
        &self.effects
    }
}

lazy_static! {
    pub static ref REGISTRY: HashMap<Id, Unit> = {
        vec![
            Unit{
                id: Id::LumberYard,
                age: Age::I,
                group: Group::RawMaterials,
                cost: Cost{
                    coins: 0,
                    resources: RMap::from([
                        (RId::Clay, 1),
                    ]),
                },
                effects: vec![],
            }
        ]
        .into_iter()
        .map(|unit| (unit.id, unit))
        .collect::<HashMap<_,_>>()
    };
}