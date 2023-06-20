use crate::base::{Player, Age, Phase, Bonus, ScientificSymbol, COINS_PER_POINT};
use crate::resource::{Map as RMap};
use crate::building::{Id as BId, List as Buildings, Group as BGroup, by_group};
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
    pub coins: u8,
    pub wonders: Wonders,
    pub buildings: Buildings,
    pub tokens: Tokens,
    pub symbols: Symbols,
    pub chains: Buildings,
}

impl City {
    pub fn bonus_rate(&self, b: &Bonus) -> u8 {
        match b {
            Bonus::Resources => self.bonus_rate(&Bonus::RawMaterials) + self.bonus_rate(&Bonus::ManufacturedGoods),
            Bonus::RawMaterials => by_group(&self.buildings, BGroup::RawMaterials).len() as u8,
            Bonus::ManufacturedGoods => by_group(&self.buildings, BGroup::ManufacturedGoods).len() as u8,
            Bonus::Military => by_group(&self.buildings, BGroup::Military).len() as u8,
            Bonus::Commercial => by_group(&self.buildings, BGroup::Commercial).len() as u8,
            Bonus::Civilian => by_group(&self.buildings, BGroup::Civilian).len() as u8,
            Bonus::Science => by_group(&self.buildings, BGroup::Scientific).len() as u8,
            Bonus::Wonder => {
                self.wonders
                    .iter()
                    .filter(|(_, bid)| bid.is_some())
                    .count() as u8
            }
            Bonus::Coin => self.coins / COINS_PER_POINT,
        }
    }
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

