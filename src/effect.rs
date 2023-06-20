use std::ops::Add;
use crate::base::{Bonus, DiscountContext, ScientificSymbol};
use crate::resource::{Id as RId, List as Resources};
use crate::state::State;
use crate::building::{Id as BId, Group as BGroup, List as Buildings, REGISTRY as BRegistry, by_group};

pub type List = Vec<&'static Effect>;

pub enum Effect {
    Chain { building: BId },
    Coins { count: u8 },
    CoinsFor { count: u8, bonus: Bonus },
    DestructBuilding { group: BGroup },
    DiscardRewardAdjuster {},
    Discounter { context: DiscountContext, resource: Resources, count: u8 },
    Fine { count: u8 },
    FixedCost { resources: Resources },
    Guild { bonus: Bonus, points: u8, coins: u8 },
    Mathematics {},
    Military { power: u8, strategy_disabled: bool },
    PickBoardToken {},
    PickDiscardedCard {},
    PickRandomToken {},
    PickReturnedCards {},
    PickTopLineCard {},
    PlayAgain {},
    Points { count: u8 },
    Resource { resource: RId, count: u8 },
    Science { symbol: ScientificSymbol },
}

impl Effect {
    pub fn apply(&'static self, s: &mut State) {
        match self {
            Self::Chain { building } => s.me.chains.push(building),
            Self::Coins { count } => s.me.coins += count,
            Self::CoinsFor { count, bonus } =>
                s.me.coins += s.me.bonus_rate(bonus) * count,
            Self::DestructBuilding {group} => {
                let buildings = by_group(&s.enemy.buildings, *group);

                if buildings.is_empty() {
                    ()
                }

                // push dialog
            }
            _ => (),
        }
    }

    pub fn discard(&self, _s: &mut State) {
        ()
    }

    pub fn points(&self, _s: &State) -> u8 {
        0
    }
}


//
// struct DestructBuilding {
//     pub group: BGroup,
// }
//
// impl Effect for DestructBuilding {
//     fn apply(&self, s: &mut State) {
//         let buildings: Buildings = s.enemy.buildings
//             .iter()
//             .filter(
//                 |bid|
//                     BRegistry
//                         .get(bid)
//                         .unwrap().group == self.group
//             )
//             .map(|bid| *bid)
//             .collect();
//
//         if buildings.len() == 0 {
//             return
//         }
//         // ()
//     }
// }