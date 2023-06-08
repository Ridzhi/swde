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
            }
        ]
        .into_iter()
        .map(|unit| (unit.id, unit))
        .collect::<HashMap<_,_>>()
    };
}