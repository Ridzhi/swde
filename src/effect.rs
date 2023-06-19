use crate::state::State;
use crate::building::{Id as BId, List as Buildings, Group as BGroup, REGISTRY as BRegistry};

pub type List = Vec<Box<dyn Effect + Sync>>;

pub trait Effect {
    fn apply(&self, s: &mut State) {
        ()
    }

    fn discard(&self, s: &mut State) {
        ()
    }

    fn points(&self, s: &State) -> u8 {
        0
    }
}

struct DestructBuilding {
    pub group: BGroup,
}

impl Effect for DestructBuilding {
    fn apply(&self, s: &mut State) {
        let buildings: Buildings = s.enemy.buildings
            .iter()
            .filter(
                |bid|
                    BRegistry
                        .get(bid)
                        .unwrap().group == self.group
            )
            .map(|bid| *bid)
            .collect();

        if buildings.len() == 0 {
            return
        }
        // ()
    }
}