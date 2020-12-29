#[derive(Eq, PartialEq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct Var {
    sym: char,
}
impl Var {
    pub fn new(c: char) -> Var {
        Var { sym: c }
    }
}

impl std::fmt::Debug for Var {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.sym)
    }
}

pub struct Ring {
    vars: Vec<Var>,
}
