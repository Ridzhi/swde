use crate::base::{Cost, Unit as BaseUnit};
use crate::resource::{Id as RId, Map as RMap};
use crate::effect::List as Effects;
use std::collections::HashMap;
use lazy_static::lazy_static;

pub type List = Vec<Id>;

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub enum Id {
    TheAppianWay = 1,
    CircusMaximus,
    TheColossus,
    TheGreatLibrary,
    TheGreatLighthouse,
    TheHangingGardens,
    TheMausoleum,
    Piraeus,
    ThePyramids,
    TheSphinx,
    TheStatueOfZeus,
    TheTempleOfArtemis,
    Messe,
    StatueOfLiberty,
}

pub struct Unit {
    pub id: Id,
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
                id: Id::TheAppianWay,
                cost: Cost{
                    coins: 0,
                    resources: RMap::from([
                        (RId::Papyrus, 1),
                        (RId::Clay, 2),
                        (RId::Stone, 2),
                    ])
                },
                effects: vec![

                ]
            },
            Unit{
                id: Id::CircusMaximus,
                cost: Cost{
                    coins: 0,
                    resources: RMap::from([
                        (RId::Glass, 1),
                        (RId::Wood, 1),
                        (RId::Stone, 2),
                    ])
                },
                effects: vec![

                ]
            },
            Unit{
                id: Id::TheColossus,
                cost: Cost{
                    coins: 0,
                    resources: RMap::from([
                        (RId::Glass, 1),
                        (RId::Clay, 1),
                    ])
                },
                effects: vec![

                ]
            },
            Unit{
                id: Id::TheGreatLibrary,
                cost: Cost{
                    coins: 0,
                    resources: RMap::from([
                        (RId::Papyrus, 1),
                        (RId::Glass, 1),
                        (RId::Wood, 3),
                    ])
                },
                effects: vec![

                ]
            },
            Unit{
                id: Id::TheGreatLighthouse,
                cost: Cost{
                    coins: 0,
                    resources: RMap::from([
                        (RId::Papyrus, 2),
                        (RId::Stone, 1),
                        (RId::Wood, 1),
                    ])
                },
                effects: vec![

                ]
            },
            Unit{
                id: Id::TheHangingGardens,
                cost: Cost{
                    coins: 0,
                    resources: RMap::from([
                        (RId::Papyrus, 1),
                        (RId::Glass, 1),
                        (RId::Wood, 2),
                    ])
                },
                effects: vec![

                ]
            },
            Unit{
                id: Id::TheMausoleum,
                cost: Cost{
                    coins: 0,
                    resources: RMap::from([
                        (RId::Papyrus, 1),
                        (RId::Glass, 2),
                        (RId::Clay, 2),
                    ])
                },
                effects: vec![

                ]
            },
            Unit{
                id: Id::Piraeus,
                cost: Cost{
                    coins: 0,
                    resources: RMap::from([
                        (RId::Clay, 1),
                        (RId::Stone, 1),
                        (RId::Wood, 2),
                    ])
                },
                effects: vec![

                ]
            },
            Unit{
                id: Id::ThePyramids,
                cost: Cost{
                    coins: 0,
                    resources: RMap::from([
                        (RId::Papyrus, 1),
                        (RId::Stone, 3),
                    ])
                },
                effects: vec![

                ]
            },
            Unit{
                id: Id::TheSphinx,
                cost: Cost{
                    coins: 0,
                    resources: RMap::from([
                        (RId::Glass, 2),
                        (RId::Clay, 1),
                        (RId::Stone, 1),
                    ])
                },
                effects: vec![

                ]
            },
            Unit{
                id: Id::TheStatueOfZeus,
                cost: Cost{
                    coins: 0,
                    resources: RMap::from([
                        (RId::Papyrus, 2),
                        (RId::Clay, 1),
                        (RId::Wood, 1),
                        (RId::Stone, 1),
                    ])
                },
                effects: vec![

                ]
            },
            Unit{
                id: Id::TheTempleOfArtemis,
                cost: Cost{
                    coins: 0,
                    resources: RMap::from([
                        (RId::Papyrus, 1),
                        (RId::Glass, 1),
                        (RId::Stone, 1),
                        (RId::Wood, 1),
                    ])
                },
                effects: vec![

                ]
            },
            Unit{
                id: Id::Messe,
                cost: Cost{
                    coins: 0,
                    resources: RMap::from([
                        (RId::Glass, 1),
                        (RId::Papyrus, 1),
                        (RId::Wood, 1),
                        (RId::Clay, 1),
                    ])
                },
                effects: vec![

                ]
            },
            Unit{
                id: Id::StatueOfLiberty,
                cost: Cost{
                    coins: 0,
                    resources: RMap::from([
                        (RId::Glass, 1),
                        (RId::Papyrus, 1),
                        (RId::Clay, 1),
                        (RId::Stone, 1),
                        (RId::Wood, 1),
                    ])
                },
                effects: vec![

                ]
            },
        ]
        .into_iter()
        .map(|unit| (unit.id, unit))
        .collect::<HashMap<_,_>>()
    };
}