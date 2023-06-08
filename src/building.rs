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
                    resources: RMap::new(),
                },
                effects: vec![],
            },
            Unit{
                id: Id::LoggingCamp,
                age: Age::I,
                group: Group::RawMaterials,
                cost: Cost{
                    coins: 1,
                    resources: RMap::new(),
                },
                effects: vec![],
            },
            Unit{
                id: Id::ClayPool,
                age: Age::I,
                group: Group::RawMaterials,
                cost: Cost{
                    coins: 0,
                    resources: RMap::new(),
                },
                effects: vec![],
            },
            Unit{
                id: Id::ClayPit,
                age: Age::I,
                group: Group::RawMaterials,
                cost: Cost{
                    coins: 1,
                    resources: RMap::new(),
                },
                effects: vec![],
            },
            Unit{
                id: Id::Quarry,
                age: Age::I,
                group: Group::RawMaterials,
                cost: Cost{
                    coins: 0,
                    resources: RMap::new(),
                },
                effects: vec![],
            },
            Unit{
                id: Id::StonePit,
                age: Age::I,
                group: Group::RawMaterials,
                cost: Cost{
                    coins: 1,
                    resources: RMap::new(),
                },
                effects: vec![],
            },
            Unit{
                id: Id::GlassWorks,
                age: Age::I,
                group: Group::ManufacturedGoods,
                cost: Cost{
                    coins: 1,
                    resources: RMap::new(),
                },
                effects: vec![],
            },
            Unit{
                id: Id::Press,
                age: Age::I,
                group: Group::ManufacturedGoods,
                cost: Cost{
                    coins: 1,
                    resources: RMap::new(),
                },
                effects: vec![],
            },
            Unit{
                id: Id::GuardTower,
                age: Age::I,
                group: Group::Military,
                cost: Cost{
                    coins: 0,
                    resources: RMap::new(),
                },
                effects: vec![],
            },
            Unit{
                id: Id::Workshop,
                age: Age::I,
                group: Group::Scientific,
                cost: Cost{
                    coins: 0,
                    resources: RMap::from([
                        (RId::Papyrus, 1),
                    ]),
                },
                effects: vec![],
            },
            Unit{
                id: Id::Apothecary,
                age: Age::I,
                group: Group::Scientific,
                cost: Cost{
                    coins: 0,
                    resources: RMap::from([
                        (RId::Glass, 1),
                    ]),
                },
                effects: vec![],
            },
            Unit{
                id: Id::StoneReserve,
                age: Age::I,
                group: Group::Commercial,
                cost: Cost{
                    coins: 3,
                    resources: RMap::new(),
                },
                effects: vec![],
            },
            Unit{
                id: Id::ClayReserve,
                age: Age::I,
                group: Group::Commercial,
                cost: Cost{
                    coins: 3,
                    resources: RMap::new(),
                },
                effects: vec![],
            },
            Unit{
                id: Id::WoodReserve,
                age: Age::I,
                group: Group::Commercial,
                cost: Cost{
                    coins: 3,
                    resources: RMap::new(),
                },
                effects: vec![],
            },
            Unit{
                id: Id::Stable,
                age: Age::I,
                group: Group::Military,
                cost: Cost{
                    coins: 0,
                    resources: RMap::from([
                        (RId::Wood, 1),
                    ]),
                },
                effects: vec![],
            },
            Unit{
                id: Id::Garrison,
                age: Age::I,
                group: Group::Military,
                cost: Cost{
                    coins: 0,
                    resources: RMap::from([
                        (RId::Clay, 1),
                    ]),
                },
                effects: vec![],
            },
            Unit{
                id: Id::Palisade,
                age: Age::I,
                group: Group::Military,
                cost: Cost{
                    coins: 2,
                    resources: RMap::new(),
                },
                effects: vec![],
            },
            Unit{
                id: Id::Scriptorium,
                age: Age::I,
                group: Group::Scientific,
                cost: Cost{
                    coins: 2,
                    resources: RMap::new(),
                },
                effects: vec![],
            },
            Unit{
                id: Id::Pharmacist,
                age: Age::I,
                group: Group::Scientific,
                cost: Cost{
                    coins: 2,
                    resources: RMap::new(),
                },
                effects: vec![],
            },
            Unit{
                id: Id::Theater,
                age: Age::I,
                group: Group::Civilian,
                cost: Cost{
                    coins: 0,
                    resources: RMap::new(),
                },
                effects: vec![],
            },
            Unit{
                id: Id::Altar,
                age: Age::I,
                group: Group::Civilian,
                cost: Cost{
                    coins: 0,
                    resources: RMap::new(),
                },
                effects: vec![],
            },
            Unit{
                id: Id::Baths,
                age: Age::I,
                group: Group::Civilian,
                cost: Cost{
                    coins: 0,
                    resources: RMap::from([
                        (RId::Stone, 1),
                    ]),
                },
                effects: vec![],
            },
            Unit{
                id: Id::Tavern,
                age: Age::I,
                group: Group::Commercial,
                cost: Cost{
                    coins: 0,
                    resources: RMap::new(),
                },
                effects: vec![],
            },

            Unit{
                id: Id::SawMill,
                age: Age::II,
                group: Group::RawMaterials,
                cost: Cost{
                    coins: 2,
                    resources: RMap::new(),
                },
                effects: vec![],
            },
            Unit{
                id: Id::BrickYard,
                age: Age::II,
                group: Group::RawMaterials,
                cost: Cost{
                    coins: 2,
                    resources: RMap::new(),
                },
                effects: vec![],
            },
            Unit{
                id: Id::ShelfQuarry,
                age: Age::II,
                group: Group::RawMaterials,
                cost: Cost{
                    coins: 2,
                    resources: RMap::new(),
                },
                effects: vec![],
            },
            Unit{
                id: Id::GlassBlower,
                age: Age::II,
                group: Group::ManufacturedGoods,
                cost: Cost{
                    coins: 0,
                    resources: RMap::new(),
                },
                effects: vec![],
            },
            Unit{
                id: Id::DryingRoom,
                age: Age::II,
                group: Group::ManufacturedGoods,
                cost: Cost{
                    coins: 0,
                    resources: RMap::new(),
                },
                effects: vec![],
            },
            Unit{
                id: Id::Walls,
                age: Age::II,
                group: Group::Military,
                cost: Cost{
                    coins: 0,
                    resources: RMap::from([
                        (RId::Stone, 2),
                    ]),
                },
                effects: vec![],
            },
            Unit{
                id: Id::Forum,
                age: Age::II,
                group: Group::Commercial,
                cost: Cost{
                    coins: 3,
                    resources: RMap::from([
                        (RId::Clay, 1),
                    ]),
                },
                effects: vec![],
            },
            Unit{
                id: Id::Caravansery,
                age: Age::II,
                group: Group::Commercial,
                cost: Cost{
                    coins: 2,
                    resources: RMap::from([
                        (RId::Glass, 1),
                        (RId::Papyrus, 1),
                    ]),
                },
                effects: vec![],
            },
            Unit{
                id: Id::CustomHouse,
                age: Age::II,
                group: Group::Commercial,
                cost: Cost{
                    coins: 4,
                    resources: RMap::new(),
                },
                effects: vec![],
            },
            Unit{
                id: Id::CourtHouse,
                age: Age::II,
                group: Group::Civilian,
                cost: Cost{
                    coins: 0,
                    resources: RMap::from([
                        (RId::Wood, 2),
                        (RId::Glass, 1),
                    ]),
                },
                effects: vec![],
            },
            Unit{
                id: Id::HorseBreeders,
                age: Age::II,
                group: Group::Military,
                cost: Cost{
                    coins: 0,
                    resources: RMap::from([
                        (RId::Clay, 1),
                        (RId::Wood, 1),
                    ]),
                },
                effects: vec![],
            },
            Unit{
                id: Id::Barracks,
                age: Age::II,
                group: Group::Military,
                cost: Cost{
                    coins: 3,
                    resources: RMap::new(),
                },
                effects: vec![],
            },
            Unit{
                id: Id::ArcheryRange,
                age: Age::II,
                group: Group::Military,
                cost: Cost{
                    coins: 0,
                    resources: RMap::from([
                        (RId::Stone, 1),
                        (RId::Wood, 1),
                        (RId::Papyrus, 1),
                    ]),
                },
                effects: vec![],
            },
            Unit{
                id: Id::ParadeGround,
                age: Age::II,
                group: Group::Military,
                cost: Cost{
                    coins: 0,
                    resources: RMap::from([
                        (RId::Clay, 2),
                        (RId::Glass, 1),
                    ]),
                },
                effects: vec![],
            },
            Unit{
                id: Id::Library,
                age: Age::II,
                group: Group::Scientific,
                cost: Cost{
                    coins: 0,
                    resources: RMap::from([
                        (RId::Stone, 1),
                        (RId::Wood, 1),
                        (RId::Glass, 1),
                    ]),
                },
                effects: vec![],
            },
            Unit{
                id: Id::Dispensary,
                age: Age::II,
                group: Group::Scientific,
                cost: Cost{
                    coins: 0,
                    resources: RMap::from([
                        (RId::Clay, 2),
                        (RId::Stone, 1),
                    ]),
                },
                effects: vec![],
            },
            Unit{
                id: Id::School,
                age: Age::II,
                group: Group::Scientific,
                cost: Cost{
                    coins: 0,
                    resources: RMap::from([
                        (RId::Wood, 1),
                        (RId::Papyrus, 2),
                    ]),
                },
                effects: vec![],
            },
            Unit{
                id: Id::Laboratory,
                age: Age::II,
                group: Group::Scientific,
                cost: Cost{
                    coins: 0,
                    resources: RMap::from([
                        (RId::Wood, 1),
                        (RId::Glass, 2),
                    ]),
                },
                effects: vec![],
            },
            Unit{
                id: Id::Statue,
                age: Age::II,
                group: Group::Civilian,
                cost: Cost{
                    coins: 0,
                    resources: RMap::from([
                        (RId::Clay, 2),
                    ]),
                },
                effects: vec![],
            },
            Unit{
                id: Id::Temple,
                age: Age::II,
                group: Group::Civilian,
                cost: Cost{
                    coins: 0,
                    resources: RMap::from([
                        (RId::Wood, 1),
                        (RId::Papyrus, 1),
                    ]),
                },
                effects: vec![],
            },
            Unit{
                id: Id::Aqueduct,
                age: Age::II,
                group: Group::Civilian,
                cost: Cost{
                    coins: 0,
                    resources: RMap::from([
                        (RId::Stone, 3),
                    ]),
                },
                effects: vec![],
            },
            Unit{
                id: Id::Rostrum,
                age: Age::II,
                group: Group::Civilian,
                cost: Cost{
                    coins: 0,
                    resources: RMap::from([
                        (RId::Stone, 1),
                        (RId::Wood, 1),
                    ]),
                },
                effects: vec![],
            },
            Unit{
                id: Id::Brewery,
                age: Age::II,
                group: Group::Commercial,
                cost: Cost{
                    coins: 0,
                    resources: RMap::new(),
                },
                effects: vec![],
            },
        ]
        .into_iter()
        .map(|unit| (unit.id, unit))
        .collect::<HashMap<_,_>>()
    };
}