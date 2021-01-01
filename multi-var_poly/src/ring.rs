#[derive(Eq, PartialEq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct Var {
    pub id: usize,
}
impl Var {
    pub fn new(i: usize) -> Var {
        Var { id: i }
    }
}

impl std::fmt::Debug for Var {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (b'x' + self.id as u8) as char)
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
    pub vars: HashMap<Var, String>,
    pub pars: HashSet<Par>,
}
impl Hash for Ring {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.vars
            .clone()
            .into_iter()
            .collect::<Vec<(Var, String)>>()
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
            vars: a.iter().map(|v| (*v, v.id.to_string())).collect(),
            pars: HashSet::new(),
        }))
    }
    pub fn pextend(&mut self, new_pars: Vec<Par>) {
        self.pars.extend(new_pars);
    }
    pub fn vextend(&mut self, s: String) {
        self.vars.insert(Var::new(self.vars.len()), s);
    }
}
