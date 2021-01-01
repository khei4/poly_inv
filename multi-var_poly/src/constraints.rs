use super::temp::*;
use std::collections::HashSet;

#[derive(PartialEq, Eq, Clone)]
pub struct PIdeal {
    pub gens: HashSet<Temp>,
}

#[derive(Clone)]
pub struct Constraint(PIdeal, PIdeal);

impl std::cmp::PartialEq for Constraint {
    fn eq(&self, other: &Self) -> bool {
        (self.0 == other.0 && self.1 == other.1) || (self.0 == other.1 && self.1 == other.0)
    }
}
impl std::cmp::Eq for Constraint {}
// Constraints
// Setのほうが良い気がするけど,Hashが危ない
#[derive(PartialEq, Eq, Clone)]
pub struct Cs {
    pub gens: Vec<Constraint>,
}
