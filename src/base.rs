use crate::resource::Map as RMap;
use crate::state::State;
use crate::effect::List as Effects;

pub type Player = String;

pub enum Age {
    I = 1,
    II,
    III,
}

pub enum Phase {
    Over = 1,
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
    coins: u8,
    resources: RMap,
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