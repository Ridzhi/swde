pub trait Mutate {
    fn mutate(&self, s: &mut State) -> Result<(), Error>;
}

#[derive(Debug)]
pub enum Error {
    Illegal
}

#[derive(Debug)]
pub struct State {
    pub coins: u8
}

impl State {
    pub fn from(actions: Vec<impl Mutate>) -> Result<State, Error> {
        let mut s = Self {
            coins: 0
        };

        for action in actions {
            match action.mutate(&mut s) {
                Ok(_) => continue,
                Err(e) => return Err(e)
            }
        }

        Ok(s)
    }
}

