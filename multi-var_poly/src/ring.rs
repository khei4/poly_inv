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
        write!(f, "x[{}]", self.id)
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
        let mut v = self
            .vars
            .clone()
            .into_iter()
            .collect::<Vec<(Var, String)>>();
        v.sort();
        v.hash(state);
        // Pについて見ると,拡大されたあとで妙に区別されてしまう
        // let mut p = self.pars.clone().into_iter().collect::<Vec<Par>>();
        // p.sort();
        // p.hash(state);
    }
}

impl Ring {
    pub fn new() -> Rc<RefCell<Ring>> {
        Rc::new(RefCell::new(Ring {
            vars: HashMap::new(),
            pars: HashSet::new(),
        }))
    }
    pub fn pextend(&mut self, new_pars: Vec<Par>) {
        self.pars.extend(new_pars);
    }
    pub fn vextend(&mut self, s: String) -> Var {
        let v = Var::new(self.vars.len());
        self.vars.insert(v, s);
        v
    }
}
