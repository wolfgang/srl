use std::fmt::{Display, Formatter, Result};

#[derive(Eq, Hash, PartialEq, Debug, Copy, Clone)]
pub enum ObjectType {
    Wall,
    Player,
    Enemy
}

impl Display for ObjectType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", self)
    }
}