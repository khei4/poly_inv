#[derive(Eq, PartialEq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct Var {
    pub id: char,
}
impl Var {
    pub fn new(c: char) -> Var {
        Var { id: c }
    }
}

impl std::fmt::Debug for Var {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

#[derive(Eq, PartialEq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct Par {
    pub id: usize,
}

impl Par {
    pub fn new(c: usize) -> Par {
        Par { id: c }
    }
}

impl std::fmt::Debug for Par {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "a[{}]", self.id)
    }
}

// TODO: vars, ParをMapで持つ
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Ring {
    pub vars: Vec<Var>,
    pub pars: Vec<Par>,
}

impl Ring {
    pub fn new(a: Vec<Var>) -> Rc<RefCell<Ring>> {
        Rc::new(RefCell::new(Ring {
            vars: a,
            pars: vec![],
        }))
    }
    pub fn pextend(&mut self, new_pars: Vec<Par>) {
        self.pars.extend(new_pars);
    }
}

impl From<Vec<Var>> for Ring {
    fn from(a: Vec<Var>) -> Ring {
        Ring {
            vars: a,
            pars: vec![],
        }
    }
}
