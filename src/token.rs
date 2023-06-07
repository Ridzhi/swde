use crate::base::{Unit as BaseUnit};
use crate::effect::List as Effects;

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

struct Unit {
    pub effects: Effects,
}

impl BaseUnit for Unit {
    fn effects(&self) -> &Effects {
        &self.effects
    }
}