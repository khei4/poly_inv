use super::coef::*;
use super::expr::*;
use super::mon::*;
use super::poly::*;
use super::ring::*;
use super::temp::*;
use std::cell::RefCell;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::rc::Rc;
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct PIdeal {
    pub gens: HashSet<Temp>,
}

impl Hash for PIdeal {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut g = self.gens.clone().into_iter().collect::<Vec<Temp>>();
        g.sort();
        g.hash(state);
    }
}

impl From<Temp> for PIdeal {
    fn from(temp: Temp) -> Self {
        let mut gens = HashSet::new();
        gens.insert(temp);
        PIdeal { gens }
    }
}

impl PIdeal {
    pub fn new() -> PIdeal {
        PIdeal {
            gens: HashSet::new(),
        }
    }
    pub fn zero(r: &Rc<RefCell<Ring>>) -> PIdeal {
        let mut i = HashSet::new();
        i.insert(Temp::zero(r));
        PIdeal { gens: i }
    }
    pub fn most_gen(d: usize, r: &Rc<RefCell<Ring>>) -> PIdeal {
        let mut gens = HashSet::new();
        gens.insert(Temp::most_gen(d, r));
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

#[derive(Clone, Debug, Hash)]
pub struct Constraint(pub PIdeal, pub PIdeal);

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
    pub items: HashSet<Constraint>,
}

impl Cs {
    pub fn new() -> Cs {
        Cs {
            items: HashSet::new(),
        }
    }
    fn union(mut self, other: Cs) -> Cs {
        self.items.extend(other.items);
        self
    }

    pub fn add(mut self, e: Constraint) -> Cs {
        self.items.insert(e);
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

// don't multiply if-guard polynomial
pub fn gen_con_less_precise(e: &Expr, mut ideal: PIdeal, mut c: Cs) -> (PIdeal, Cs) {
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
                Pred { p, eq } if *eq => (i1.rem_par(p).union(i2), c1.union(c2)),
                Pred { p, .. } => (i2.rem_par(p).union(i1), c1.union(c2)),
            }
        }
        Expr::While { c: body, .. } => {
            let (i1, c1) = gen_con(body, ideal.clone(), c.clone());
            c = c.add(Constraint(ideal.clone(), i1));
            (ideal, c.union(c1))
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct LinearEquations {
    parsize: usize,
    eqs: HashSet<(LinExp, C)>,
}

impl std::fmt::Display for LinearEquations {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res = String::new();
        for (le, c) in &self.eqs {
            let mut outvec: Vec<C> = vec![C::zero(); self.parsize];
            for pt in &le.terms {
                match pt.par {
                    Some(p) => outvec[p.id] = pt.coef,
                    None => (),
                }
            }
            for i in 0..self.parsize {
                let term;
                if outvec[i] == C::zero() {
                    term = String::new();
                } else if outvec[i] == C::one() {
                    term = format!("{:?}", Par::new(i));
                } else if outvec[i] == -C::one() {
                    term = format!("-{:?}", Par::new(i));
                } else {
                    term = format!("{}{:?}", outvec[i], Par::new(i));
                }
                res = format!("{}{:^7}", res, term);
            }
            res = format!("{}=  {}  \n", res, c);
        }
        write!(f, "{}", res)
    }
}

/*
*/

impl From<(Cs, &Rc<RefCell<Ring>>)> for LinearEquations {
    fn from(cs_r: (Cs, &Rc<RefCell<Ring>>)) -> Self {
        let (cs, r) = cs_r;
        let mut eqs = HashSet::new();
        for c in cs.items {
            let (left_pideal, right_pideal) = (c.0, c.1);
            for t1 in &right_pideal.gens {
                for t2 in &left_pideal.gens {
                    // 係数一致
                    let t = t1.clone() + -t2.clone();
                    // ゼロにならない
                    if t.mons
                        .last()
                        .expect("mons length 0 at eqs")
                        .0
                        .coef
                        .is_cnst()
                    {
                        panic!("solution does'nt exist");
                    }

                    for m in &t.mons {
                        let mut le = m.0.coef.clone();
                        let mut cnst = C::zero();
                        if le.terms[0].is_cnst() {
                            cnst = -le.terms[0].coef;
                            le.terms.remove(0);
                        }
                        eqs.insert((le, cnst));
                    }
                }
            }
        }
        LinearEquations {
            parsize: r.borrow().pars.len(),
            eqs,
        }
    }
}

#[test]
fn zero_and_mostgen() {
    // 0 -> x, 1 -> y, 2 -> z
    let x0 = Var::new(0);
    let x1 = Var::new(1);
    let x2 = Var::new(2);
    let vars = vec![x0, x1, x2];
    let r = Ring::new(vars);
    let i = PIdeal::most_gen(1, &r);

    let mut a0x0: Mon<LinExp> = Mon::from((Par::new(0), vec![(x0, 1)]));
    a0x0.coef += LinExp::one();
    let mut a1x1: Mon<LinExp> = Mon::from((Par::new(1), vec![(x1, 1)]));
    a1x1.coef += LinExp::one() * (C::one() * 8);
    let a2x2: Mon<LinExp> = Mon::from((Par::new(2), vec![(x2, 1)]));
    let t = Temp::from((vec![a0x0, a1x1, a2x2], &r));
    println!("{:?}", t);
    println!("{:?}", i.gens);
    let eq_cons = Constraint(i, PIdeal::from(t));
    let mut c = Cs::new();
    c = c.add(eq_cons);

    let leq = LinearEquations::from((c, &r));
    println!("{}", leq);
}
