use crate::state::State;
use crate::building::{Id as BId, Group as BGroup};

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
        ()
    }
}