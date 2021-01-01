use super::coef::*;
use super::mon::*;
use super::poly::*;
use super::ring::*;
use itertools::Itertools;
use std::cell::RefCell;
use std::cmp::Reverse;
use std::hash::{Hash, Hasher};
use std::rc::Rc;
#[derive(PartialEq, Clone)]
pub struct Temp {
    pub mons: Vec<Reverse<Mon<LinExp>>>,
    pub r: Rc<RefCell<Ring>>,
}
impl Eq for Temp {}
impl Hash for Temp {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.mons.hash(state);
    }
}

impl std::fmt::Debug for Temp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut res = format!("{:?}", self.mons[0].0);
        for i in 1..self.mons.len() {
            if self.mons[i].0.coef >= LinExp::zero() {
                res = format!("{}+{:?}", res, self.mons[i].0);
            } else {
                res = format!("{}-{:?}", res, self.mons[i].0);
            }
        }
        write!(f, "{}", res)
    }
}

// constructers
impl Temp {
    fn zero(r: Rc<RefCell<Ring>>) -> Temp {
        Temp {
            mons: vec![Reverse(Mon::zero())],
            r,
        }
    }
    fn one(r: Rc<RefCell<Ring>>) -> Temp {
        Temp {
            mons: vec![Reverse(Mon::one())],
            r,
        }
    }
}

#[test]
fn zero_is_identity_of_add() {
    let r = Ring::new(vec![]);
    assert!(Temp::one(r.clone()) + Temp::zero(r.clone()) == Temp::one(r.clone()));
    assert!(Temp::zero(r.clone()) + Temp::one(r.clone()) == Temp::one(r.clone()));
    assert!(Temp::zero(r.clone()) + Temp::zero(r.clone()) == Temp::zero(r.clone()));
}

// construct Temp with extend Rings
impl From<(Vec<Mon<LinExp>>, Rc<RefCell<Ring>>)> for Temp {
    fn from(a: (Vec<Mon<LinExp>>, Rc<RefCell<Ring>>)) -> Self {
        let (a, r) = a;
        let mut mons = vec![];
        // TODO: parametersをSetにする.
        for m in a {
            for pt in &m.coef.terms {
                match pt.par {
                    Some(p) => r.borrow_mut().pars.push(p),
                    None => (),
                }
            }
            mons.push(Reverse(m));
        }
        r.borrow_mut().pars.sort();
        r.borrow_mut().pars.dedup();
        if mons.len() == 0 {
            mons.push(Reverse(Mon::zero()));
        }
        let mut p = Temp { mons, r };
        p.sort_sumup();
        p
    }
}

// methods
impl Temp {
    fn is_zero(&self) -> bool {
        self.mons[0].0 == Mon::zero()
    }
    fn sort_sumup(&mut self) {
        // dummy monomial
        let dm = Reverse(Mon::<LinExp>::zero());
        self.mons.push(dm.clone());
        // 0を追加して, 最後にまとめて消す
        self.mons.sort();
        for i in 1..self.mons.len() {
            // TODO: 突貫工事, どこかで生じる空リスト係数をなくせ
            if self.mons[i].0.coef.terms.len() == 0 {
                self.mons[i] = dm.clone();
            }
            if !(self.mons[i - 1] > self.mons[i]) && !(self.mons[i - 1] < self.mons[i]) {
                let c = self.mons[i].0.coef.clone();
                self.mons[i - 1].0.coef += c;
                self.mons[i] = dm.clone();
                if self.mons[i - 1].0.coef == LinExp::zero() {
                    self.mons[i - 1] = dm.clone();
                }
            }
        }
        self.mons.sort();
        while let Some(m) = self.mons.pop() {
            if m != dm {
                self.mons.push(m);
                break;
            }
        }
        if self.mons.len() == 0 {
            self.mons.push(dm);
        }
    }
    // x に関しての昇順でソート
    fn sort_by_var(&mut self, v: Var) {
        self.mons.sort_by(|m1, m2| {
            m1.0.vars
                .get(&v)
                .unwrap_or(&0)
                .cmp(m2.0.vars.get(&v).unwrap_or(&0))
        });
    }
    fn tdeg(&self) -> usize {
        // minimium == Reveresed maximum
        let m = self.mons.iter().min().expect("Temp T-degree Panic");
        m.0.vars.iter().fold(0, |s, (_, v)| s + v)
    }

    fn most_gen(d: usize, r: Rc<RefCell<Ring>>) -> Temp {
        let v = r.borrow_mut().vars.clone();
        let mut cnt = r.borrow().pars.len();
        let mut fresh_pars = vec![];
        let mut mons = vec![];
        for i in 0..d + 1 {
            for c in v.iter().combinations_with_replacement(i) {
                // これはケッコーやばい
                let fp = Par::new(cnt);
                cnt += 1;
                fresh_pars.push(fp);
                // varsのマップを作る
                let mut m: std::collections::HashMap<Var, usize> = std::collections::HashMap::new();
                for v in c {
                    match m.get_mut(&v) {
                        Some(d) => *d += 1,
                        None => {
                            m.insert(*v, 1);
                        }
                    }
                }
                mons.push(Reverse(Mon::<LinExp>::from((fp, m))))
            }
        }
        r.borrow_mut().pextend(fresh_pars);
        mons.sort();
        Temp { mons, r: r }
    }

    fn rem_par(&self, other: Poly) -> Temp {
        let diff = self.tdeg() - other.tdeg();
        let q = Temp::most_gen(diff, self.r.clone());
        q * (-other) + self.clone()
    }

    fn subs(mut self, v: Var, other: Poly) -> Temp {
        self.sort_by_var(v);
        let mut res = Temp::zero(self.r.clone());
        let mut base = Poly::one(self.r.clone());
        let mut cur = 0;
        for m in &mut self.mons {
            match m.0.vars.remove(&v) {
                Some(d) => {
                    assert!(d > 0);
                    if cur < d {
                        base *= other.pow(d - cur);
                        cur = d;
                    }
                }
                None => (),
            }
            res += Temp::from((vec![m.0.clone()], self.r.clone())) * base.clone();
        }
        res
    }
}

impl std::ops::Add<Temp> for Temp {
    type Output = Temp;

    fn add(mut self, rhs: Temp) -> Self::Output {
        self.mons.extend(rhs.mons);
        self.sort_sumup();
        self
    }
}

impl std::ops::AddAssign<Temp> for Temp {
    fn add_assign(&mut self, rhs: Temp) {
        *self = self.clone() + rhs;
    }
}

impl std::ops::Mul<Poly> for Temp {
    type Output = Temp;
    fn mul(mut self, other: Poly) -> Self::Output {
        if other.is_zero() || self.is_zero() {
            Temp::zero(self.r.clone())
        } else {
            let mut new_terms: Vec<Reverse<Mon<LinExp>>> = vec![];
            for m in &other.mons {
                for tm in &self.mons {
                    new_terms.push(Reverse(tm.0.clone() * m.0.clone()))
                }
            }
            if new_terms.len() == 0 {
                Temp::zero(self.r.clone())
            } else {
                self.mons = new_terms;
                self.sort_sumup();
                self
            }
        }
    }
}

#[test]
fn one_id_of_mul_zero_is_zero() {
    let r = Ring::new(vec![]);
    assert!(Temp::one(r.clone()) * Poly::zero(r.clone()) == Temp::zero(r.clone()));
    println!("T 1 * P 0 = T 0");
    assert!(Temp::zero(r.clone()) * Poly::one(r.clone()) == Temp::zero(r.clone()));
    println!("T 0 * P 1 = T 0");
    assert!(Temp::one(r.clone()) * Poly::one(r.clone()) == Temp::one(r.clone()));
    println!("T 1 * P 1 = T 1");
    assert!(Temp::zero(r.clone()) * Poly::zero(r.clone()) == Temp::zero(r.clone()));
    println!("T 0 * P 0 = T 0");
}

impl std::ops::MulAssign<Poly> for Temp {
    fn mul_assign(&mut self, other: Poly) {
        *self = self.clone() * other;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_temp_add_poly_mul() {
        // Init Ring
        let x: Var = Var::new('x');
        let y = Var::new('y');
        let z = Var::new('z');
        let r = Ring::new(vec![x, y, z]);
        // parameters
        let pars: Vec<Par> = (0..4).map(|i| Par::new(i)).collect();
        // Init Template

        let ax2: Mon<LinExp> = Mon::from((pars[0], vec![(x, 2)]));
        let bx2: Mon<LinExp> = Mon::from((pars[1], vec![(x, 1), (y, 1)]));
        let cxy: Mon<LinExp> = Mon::from((pars[2], vec![(x, 1), (y, 1)]));
        let dxy: Mon<LinExp> = Mon::from((pars[3], vec![(x, 1), (y, 1)]));
        let y2: Mon<LinExp> = Mon::from((pars[3], vec![(y, 2)]));
        let yz: Mon<LinExp> = Mon::from((pars[0], vec![(y, 1), (z, 1)]));
        let p1 = Temp::from((vec![ax2, cxy, yz, y2.clone()], r.clone()));
        let p2 = Temp::from((vec![bx2, dxy, y2], r.clone()));
        // もれなくだぶりなく拡張されている
        assert!(r.as_ref().borrow().pars == pars);
        assert!(p1.tdeg() == 2);
        assert!(p2.tdeg() == 2);

        /*
            Addition Test
        */

        let p1 = p1 + p2;

        assert!(p1.clone() == p1.clone() + Temp::zero(r.clone()));
        assert!(p1.clone() == Temp::zero(r.clone()) + p1.clone());

        // Monomials, Polynomials
        let x2: Mon<C> = Mon::from(vec![(x, 2)]);
        let xy: Mon<C> = Mon::from(vec![(x, 1), (y, 1)]);
        let y2: Mon<C> = Mon::from(vec![(y, 2)]);
        let yz: Mon<C> = Mon::from(vec![(y, 1), (z, 1)]);
        let twelve: Mon<C> = Mon::one() * C::new(12, 1);
        let p2 = Poly::from((vec![x2, yz, xy, y2, twelve], r.clone()));

        /*
            Multiplication test
        */
        assert!(p1.tdeg() == 2);
        let m = p1 * p2;
        println!("{:?}", m);
        assert!(m.tdeg() == 4);
    }

    #[test]
    fn check_subs_mostgen() {
        // Init Ring
        let x: Var = Var::new('x');
        let y = Var::new('y');
        let z = Var::new('z');
        let r = Ring::new(vec![x, y, z]);

        // Init Template by
        /*
            Most Generic Template
        */
        let p1 = Temp::most_gen(2, r.clone());
        // 3 variable, 2 degree => 4H2 == 5C2 == 10
        assert!(r.as_ref().borrow().pars.len() == 10);
        println!("{:?}", p1);
        assert!(p1.tdeg() == 2);

        // Monomials
        let x2: Mon<C> = Mon::from(vec![(x, 2)]);
        let xy: Mon<C> = Mon::from(vec![(x, 1), (y, 1)]);
        let y2: Mon<C> = Mon::from(vec![(y, 2)]);
        let yz: Mon<C> = Mon::from(vec![(y, 1), (z, 1)]);
        let twelve: Mon<C> = Mon::one() * C::new(12, 1);
        let p2 = Poly::from((vec![x2, yz, y2, twelve], r.clone()));

        /*
            Substitution Test
        */
        println!("{:?} subs {:?} to {:?} ", p1, x, p2);
        println!("{:?}", p1.subs(x, p2));
    }

    #[test]
    fn check_mannadiv_poly() {
        /*
            Devide concrete polynomial invariant by guard, assignment polynomials
        */
        // Init Ring
        // v -> x1, w -> x2, x -> y1, y -> y2, z -> y3
        let x1 = Var::new('v');
        let x2 = Var::new('w');
        let y1 = Var::new('x');
        let y2 = Var::new('y');
        let y3 = Var::new('z');
        let r = Ring::new(vec![x1, x2, y1, y2, y3]);
        // Init Invariant (Template)
        // y1*x2 + y2 + y3 - x1 = 0

        let m1: Mon<LinExp> = Mon::from(vec![(y1, 1), (x2, 1)]);
        let m2: Mon<LinExp> = Mon::from(vec![(y2, 1)]);
        let m3: Mon<LinExp> = Mon::from(vec![(y3, 1)]);
        let m4: Mon<LinExp> = Mon::from(vec![(x1, 1)]) * -C::one();
        let g_inv = Temp::from((vec![m1, m2, m3, m4], r.clone()));

        // guard polynomial
        // p = x2-y2-1
        let m1: Mon<C> = Mon::from(vec![(x2, 1)]);
        let m2: Mon<C> = Mon::from(vec![(y2, 1)]) * -C::one();
        let n_one: Mon<C> = Mon::one() * -C::one();
        let p = Poly::from((vec![m1, m2, n_one], r.clone()));

        // subs poly pcxyVn => cx's y-th substitution to Vn variable
        let pc11y1 = Poly::from((vec![Mon::from(vec![(y1, 1)]), Mon::one()], r.clone()));
        let pc12y2 = Poly::zero(r.clone());
        let pc13y3 = Poly::from((
            vec![Mon::from(vec![(y3, 1)]), Mon::one() * -C::one()],
            r.clone(),
        ));
        let pc21y2 = Poly::from((vec![Mon::from(vec![(y2, 1)]), Mon::one()], r.clone()));
        let pc22y3 = Poly::from((
            vec![Mon::from(vec![(y3, 1)]), Mon::one() * -C::one()],
            r.clone(),
        ));
        let c1g = {
            let subs1 = {
                let subs2 = {
                    let subs3 = g_inv.clone().subs(y3, pc13y3);
                    subs3.subs(y2, pc12y2)
                };
                subs2.subs(y1, pc11y1)
            };
            subs1
        };
        println!("c1g{:?}", c1g);
        println!("g_inv{:?}", g_inv);
        let c2g = {
            let subs1 = {
                let subs2 = g_inv.clone().subs(y3, pc22y3);
                subs2.subs(y2, pc21y2)
            };
            subs1
        };
        println!("{:?}", c2g.mons[c2g.mons.len() - 1].0.coef.terms);
        println!("c2g{:?}", c2g);
        println!("g_inv{:?}", g_inv);
        assert!(c2g == g_inv);
        let remainder = c1g.rem_par(p.clone());
        println!("{:?}", remainder);
        let pg = c2g * p;

        // last substitution
        // pxVn => x-th substitution to Vn
        let p1y1 = Poly::zero(r.clone());
        let p2y2 = Poly::zero(r.clone());
        let p3y3 = Poly::from((vec![Mon::from(vec![(x1, 1)])], r.clone()));
        let g1 = {
            let subs1 = {
                let subs2 = {
                    let subs3 = pg.subs(y3, p3y3.clone());
                    subs3.subs(y2, p2y2.clone())
                };
                subs2.subs(y1, p1y1.clone())
            };
            subs1
        };
        let g2 = {
            let subs1 = {
                let subs2 = {
                    let subs3 = remainder.subs(y3, p3y3);
                    subs3.subs(y2, p2y2)
                };
                subs2.subs(y1, p1y1)
            };
            subs1
        };
        println!("{:?}", g1);
        println!("{:?}", g2);
    }
}
