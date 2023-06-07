use std::collections::HashMap;

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub enum Id {
    Clay = 1,
    Wood,
    Stone,
    Glass,
    Papyrus,
}

pub type Map = HashMap<Id, u16>;