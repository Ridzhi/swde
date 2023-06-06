use crate::State;
use crate::state::{Mutate, Error};

pub struct ConstructBuilding {
    coins: u8
}

impl ConstructBuilding {
    pub fn new(coins: u8) -> Self {
        Self{
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