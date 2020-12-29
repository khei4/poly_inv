#[derive(Eq, PartialEq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct Var {
    pub sym: char,
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

#[derive(Eq, PartialEq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct Par {
    pub sym: char,
}

impl Par {
    pub fn new(c: char) -> Par {
        Par { sym: c }
    }
}

impl std::fmt::Debug for Par {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.sym)
    }
}

#[derive(Eq, PartialEq, Clone)]
pub struct Ring {
    vars: Vec<Var>,
    pars: Vec<Par>,
}
