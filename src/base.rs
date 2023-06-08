use crate::resource::Map as RMap;
use crate::state::State;
use crate::effect::List as Effects;

pub type Player = String;

#[derive(Debug, Default)]
pub enum Age {
    #[default]
    I = 1,
    II,
    III,
}

#[derive(Debug, Default)]
pub enum Phase {
    Over = 1,
    #[default]
    WondersSelection,
    Turn,
    WhoBeginsTheNextAgeSelection,
    BoardTokenSelection,
    RandomTokenSelection,
    DiscardBuildingSelection,
    DiscardedBuildingSelection,
    TopLineBuildingSelection,
    ReturnedBuildingSelection,
}

#[derive(Debug)]
pub enum Bonus {
    Resources = 1,
    RawMaterials,
    ManufacturedGoods,
    Military,
    Commercial,
    Civilian,
    Science,
    Wonder,
    Coin,
}

#[derive(Debug)]
pub enum ScientificSymbol {
    Astrology = 1,
    Wheel,
    Sundial,
    Mortar,
    Compass,
    Writing,
    Law,
}

pub struct Cost {
    pub coins: u8,
    pub resources: RMap,
}

pub trait Unit {
    fn effects(&self) -> &Effects;

    fn construct(&self, s: &mut State) {
        for effect in self.effects() {
            effect.apply(s)
        }
    }

    fn destruct(&self, s: &mut State) {
        for effect in self.effects() {
            effect.discard(s)
        }
    }

    fn points(&self, s: &State) -> u8 {
        let mut sum: u8 = 0;

        for effect in self.effects() {
            sum += effect.points(s)
        }

        sum
    }
}