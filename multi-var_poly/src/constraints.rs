use super::expr::*;
use super::poly::*;
use super::ring::*;
use super::temp::*;
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct PIdeal {
    pub gens: HashSet<Temp>,
}
impl PIdeal {
    pub fn new() -> PIdeal {
        PIdeal {
            gens: HashSet::new(),
        }
    }
    pub fn most_gen(d: usize, r: &Rc<RefCell<Ring>>) -> PIdeal {
        let mut gens = HashSet::new();
        gens.insert(Temp::most_gen(d, r.clone()));
        PIdeal { gens }
    }
}

impl PIdeal {
    fn union(mut self, other: PIdeal) -> PIdeal {
        self.gens.union(&other.gens);
        self
    }

    fn rem_par(mut self, divisor: &Poly) -> PIdeal {
        let mut new_gens = HashSet::new();
        for g in self.gens {
            new_gens.insert(g.rem_par(divisor.clone()));
        }
        self.gens = new_gens;
        self
    }

    fn mul(mut self, other: &Poly) -> PIdeal {
        let mut new_gens = HashSet::new();
        for g in self.gens {
            new_gens.insert(g * other.clone());
        }
        self.gens = new_gens;
        self
    }
}
// impl Iterator for PIdeal {
//     type Item = Temp;

//     fn next(&mut self) -> Option<Self::Item> {

//     }
// }

#[derive(Clone, Debug)]
pub struct Constraint(PIdeal, PIdeal);

impl std::cmp::PartialEq for Constraint {
    fn eq(&self, other: &Self) -> bool {
        (self.0 == other.0 && self.1 == other.1) || (self.0 == other.1 && self.1 == other.0)
    }
}
impl std::cmp::Eq for Constraint {}
// Constraints

// Setのほうが良い気がするけど,Hashが危ない
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Cs {
    pub items: Vec<Constraint>,
}

impl Cs {
    pub fn new() -> Cs {
        Cs { items: vec![] }
    }
    fn union(mut self, other: Cs) -> Cs {
        self.items.extend(other.items);
        // TODO: 重複は一旦許す
        // self.items.sort();
        // self.items.dedup();
        self
    }

    fn add(mut self, e: Constraint) -> Cs {
        self.items.push(e);
        self
    }
}

// Generating Constraints
pub fn gen_con(e: &Expr, mut ideal: PIdeal, mut c: Cs) -> (PIdeal, Cs) {
    match e {
        Expr::Ass { lv, rv } => {
            let mut new_gens = HashSet::new();
            for tp in &mut ideal.gens.iter() {
                new_gens.insert(tp.clone().subs(*lv, rv.clone()));
            }
            ideal.gens = new_gens;
            (ideal, c)
        }
        Expr::Skip => (ideal, c),
        Expr::Seq { exprs } => {
            for i in (0..exprs.len()).rev() {
                let next_ic = gen_con(&exprs[i], ideal, c);
                ideal = next_ic.0;
                c = next_ic.1;
            }
            (ideal, c)
        }
        Expr::If { guard, the, els } => {
            let (i1, c1) = gen_con(the, ideal.clone(), c.clone());
            let (i2, c2) = gen_con(els, ideal, c);
            match guard {
                Pred { p, eq } if *eq => (i1.rem_par(p).union(i2.mul(p)), c1.union(c2)),
                Pred { p, .. } => (i2.rem_par(p).union(i1.mul(p)), c1.union(c2)),
            }
        }
        Expr::While { c: body, .. } => {
            let (i1, c1) = gen_con(body, ideal.clone(), c.clone());
            c = c.add(Constraint(ideal.clone(), i1));
            (ideal, c.union(c1))
        }
    }
}
