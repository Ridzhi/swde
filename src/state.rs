use crate::base::{Player, Age, Phase, Bonus, ScientificSymbol};
use crate::resource::{Map as RMap};
use crate::building::{Id as BId, List as Buildings};
use crate::wonder::{Id as WId};
use crate::token::{Id as TId, List as Tokens};

pub trait Mutate {
    fn mutate(&self, s: &mut State) -> Result<(), Error>;
}

#[derive(Debug)]
pub enum Error {
    Illegal
}

#[derive(Default, Debug)]
pub struct State {
    pub me: City,
    pub enemy: City,
    age: Age,
    phase: Phase,
    first_turn: Player,
}

#[derive(Debug)]
pub struct City {
    player: Player,
    pub score: Score,
    pub resources: RMap,
    pub coins: u16,
    pub wonders: Wonders,
    pub buildings: Buildings,
    pub tokens: Tokens,
    pub symbols: Symbols,
    pub chains: Buildings,
}

impl Default for City {
    fn default() -> Self {
        Self {
            player: "".to_string(),
            score: Default::default(),
            resources: Default::default(),
            coins: 0,
            wonders: vec![],
            buildings: vec![],
            tokens: vec![],
            symbols: vec![],
            chains: vec![],
        }
    }
}

#[derive(Debug, Default)]
pub struct Score {
    civilian: u8,
    science: u8,
    commercial: u8,
    guilds: u8,
    wonders: u8,
    tokens: u8,
    coins: u8,
    military: u8,
    total: u8,
}

type Wonders = Vec<(WId, Option<BId>)>;
type Symbols = Vec<ScientificSymbol>;

impl State {
    pub fn from(actions: Vec<impl Mutate>) -> Result<State, Error> {
        let mut s = Self::default();

        for action in actions {
            action.mutate(&mut s)?;
        }

        Ok(s)
    }
}

