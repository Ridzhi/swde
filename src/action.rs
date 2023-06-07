use crate::base::{Player};
use crate::state::{State, Mutate, Error};

enum Id {
    Prepare = 1,
    Over,
    SelectWhoBeginsTheNextAge,
    ConstructWonder,
    ConstructBuilding,
    DiscardBuilding,
    DestructBuilding,
    PickWonder,
    PickBoardToken,
    PickRandomToken,
    PickTopLineCard,
    PickDiscardedCard,
    PickReturnedCards,
}

pub struct Prepare {
    id: Id,
    p1: Player,
    p2: Player,
}

impl Prepare {
    pub fn new(p1: Player, p2: Player) -> Self {
        Self{
            id: Id::Prepare,
            p1,
            p2,
        }
    }
}

impl Mutate for Prepare {
    fn mutate(&self, s: &mut State) -> Result<(), Error> {
        todo!()
    }
}

pub struct ConstructBuilding {
    coins: u8,
}

impl ConstructBuilding {
    pub fn new(coins: u8) -> Self {
        Self {
            coins,
        }
    }
}

impl Mutate for ConstructBuilding {
    fn mutate(&self, s: &mut State) -> Result<(), Error> {
        s.coins += self.coins;

        Ok(())
    }
}