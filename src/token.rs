use crate::base::{Cost, Unit as BaseUnit};
use crate::resource::{Id as RId, Map as RMap};
use crate::effect::List as Effects;
use std::collections::HashMap;
use lazy_static::lazy_static;

pub type List = Vec<Id>;

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub enum Id {
    Agriculture = 1,
    Architecture,
    Economy,
    Law,
    Masonry,
    Mathematics,
    Philosophy,
    Strategy,
    Theology,
    Urbanism,
}

pub struct Unit {
    pub id: Id,
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
                id: Id::Agriculture,
                effects: vec![],
            },
            Unit{
                id: Id::Architecture,
                effects: vec![],
            },
            Unit{
                id: Id::Economy,
                effects: vec![],
            },
            Unit{
                id: Id::Law,
                effects: vec![],
            },
            Unit{
                id: Id::Masonry,
                effects: vec![],
            },
            Unit{
                id: Id::Mathematics,
                effects: vec![],
            },
            Unit{
                id: Id::Philosophy,
                effects: vec![],
            },
            Unit{
                id: Id::Strategy,
                effects: vec![],
            },
            Unit{
                id: Id::Theology,
                effects: vec![],
            },
            Unit{
                id: Id::Urbanism,
                effects: vec![],
            },
        ]
        .into_iter()
        .map(|unit| (unit.id, unit))
        .collect::<HashMap<_,_>>()
    };
}