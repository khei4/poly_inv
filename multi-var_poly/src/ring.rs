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

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::rc::Rc;
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Ring {
    pub vars: HashMap<String, Var>,
    pub pars: HashSet<Par>,
}
impl Hash for Ring {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.vars
            .clone()
            .into_iter()
            .collect::<Vec<(String, Var)>>()
            .hash(state);
        self.pars
            .clone()
            .into_iter()
            .collect::<Vec<Par>>()
            .hash(state);
    }
}

impl Ring {
    pub fn new(a: Vec<Var>) -> Rc<RefCell<Ring>> {
        Rc::new(RefCell::new(Ring {
            vars: a.iter().map(|v| (v.id.to_string(), *v)).collect(),
            pars: HashSet::new(),
        }))
    }
    pub fn pextend(&mut self, new_pars: Vec<Par>) {
        self.pars.extend(new_pars);
    }
}
