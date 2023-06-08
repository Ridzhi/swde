use crate::base::{Player};
use crate::state::{State, Mutate, Error};
use crate::wonder::{Id as WId, List as Wonders};
use crate::token::{Id as TId, List as Tokens};
use crate::building::{Id as BId, List as Buildings};
use rand;


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
    // wonders: Wonders,
    // board_tokens: Tokens,
    // random_tokens: Tokens,
    // buildings: Buildings,
}

impl Prepare {
    pub fn new(mut p1: Player, mut p2: Player) -> Self {
        if rand::random() {
            (p1, p2) = (p2, p1);
        }

        Self {
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
    coins: u16,
}

impl ConstructBuilding {
    pub fn new(coins: u16) -> Self {
        Self {
            coins,
        }
    }
}

impl Mutate for ConstructBuilding {
    fn mutate(&self, s: &mut State) -> Result<(), Error> {
        s.me.coins += self.coins;

        Ok(())
    }
}
