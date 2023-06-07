use crate::state::State;

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