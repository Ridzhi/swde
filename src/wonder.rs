use crate::base::{Cost, Unit as BaseUnit};
use crate::effect::List as Effects;

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