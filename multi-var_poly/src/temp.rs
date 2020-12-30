use super::coef::*;
use super::mon::*;
use super::poly::*;
use super::ring::*;
use itertools::Itertools;
use std::cell::RefCell;
use std::cmp::Reverse;
use std::rc::Rc;
#[derive(PartialEq, Clone)]
pub struct Temp {
    pub mons: Vec<Reverse<Mon<LinExp>>>,
    pub r: Rc<RefCell<Ring>>,
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
// construct Temp with extend Rings
impl From<(Vec<Mon<LinExp>>, Rc<RefCell<Ring>>)> for Temp {
    fn from(a: (Vec<Mon<LinExp>>, Rc<RefCell<Ring>>)) -> Self {
        let mut mons = vec![];
        for m in a.0 {
            mons.push(Reverse(m));
        }
        let mut p = Temp { mons, r: a.1 };
        p.sort_sumup();
        p
    }
}

// methods
impl Temp {
    fn sort_sumup(&mut self) {
        // dummy monomial
        let dm = Reverse(Mon::<LinExp>::zero());
        self.mons.push(dm.clone());
        // 0を追加して, 最後にまとめて消す
        self.mons.sort();
        for i in 1..self.mons.len() {
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
    }
    // x に関しての降順でソート
    fn sort_by_var(&mut self, v: Var) {
        self.mons.sort_by(|m1, m2| {
            m1.0.vars
                .get(&v)
                .unwrap_or(&0)
                .cmp(m2.0.vars.get(&v).unwrap_or(&0))
        });
    }
    fn tdeg(&self) -> usize {
        self.mons[0].0.vars.iter().fold(0, |s, (_, v)| s + v)
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
        // vに関して昇順でソート
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
            // TODO: Temp(Poly) * Mon<LinExp>
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
        let mut new_terms: Vec<Reverse<Mon<LinExp>>> = vec![];
        for m in &other.mons {
            for tm in &self.mons {
                new_terms.push(Reverse(tm.0.clone() * m.0.clone()))
            }
        }
        self.mons = new_terms;
        self.sort_sumup();
        self
    }
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
        use std::collections::HashMap;
        // Init Ring
        let x: Var = Var::new('x');
        let y = Var::new('y');
        let z = Var::new('z');
        let r = Ring::new(vec![x, y, z]);

        // Init Template
        let mut md1 = HashMap::new();
        md1.insert(x, 2);
        let mut md2 = HashMap::new();
        md2.insert(x, 1);
        md2.insert(y, 1);
        let mut md3 = HashMap::new();
        md3.insert(y, 2);
        let mut md4 = HashMap::new();
        md4.insert(y, 1);
        md4.insert(z, 1);
        let pars: Vec<Par> = (0..4).map(|i| Par::new(i)).collect();

        let yz: Mon<LinExp> = Mon::from((pars[0], md4.clone()));
        let ax2: Mon<LinExp> = Mon::from((pars[0], md1.clone()));
        let bx2: Mon<LinExp> = Mon::from((pars[1], md1.clone()));
        let cxy: Mon<LinExp> = Mon::from((pars[2], md2.clone()));
        let dxy: Mon<LinExp> = Mon::from((pars[3], md2.clone()));
        let y2: Mon<LinExp> = Mon::from((pars[3], md3.clone()));
        // TODO: ここはFromでパラメーター拡張してほしい
        r.borrow_mut().pextend(pars);
        let p1 = Temp::from((vec![ax2, cxy, yz, y2.clone()], r.clone()));
        let p2 = Temp::from((vec![bx2, dxy, y2], r.clone()));
        assert!(p1.tdeg() == 2);
        assert!(p2.tdeg() == 2);

        /*
            Addition Test
        */

        let p1 = p1 + p2;
        // Monomials, Polynomials
        let yz: Mon<C> = Mon::from(md4);
        let x2: Mon<C> = Mon::from(md1);
        let xy: Mon<C> = Mon::from(md2);
        let y2: Mon<C> = Mon::from(md3);
        let twelve: Mon<C> = Mon::one() * C::new(12, 1);
        let p2 = Poly::from((vec![x2, yz, xy, y2, twelve], r.clone()));
        assert!(p1.tdeg() == 2);
        let m = p1 * p2;
        println!("{:?}", m);
        assert!(m.tdeg() == 4);
    }

    #[test]
    fn check_subs_mostgen_rempar() {
        use std::collections::HashMap;
        // Init Ring
        let x: Var = Var::new('x');
        let y = Var::new('y');
        let z = Var::new('z');
        let r = Ring::new(vec![x, y, z]);
        // Init Monomial Dic
        let mut md1 = HashMap::new();
        md1.insert(x, 2);
        let mut md2 = HashMap::new();
        md2.insert(x, 1);
        md2.insert(y, 1);
        let mut md3 = HashMap::new();
        md3.insert(y, 2);
        let mut md4 = HashMap::new();
        md4.insert(y, 1);
        md4.insert(z, 1);

        // Init Template by
        /*
            Most Generic Template
        */
        let p1 = Temp::most_gen(2, r.clone());
        println!("{:?}", p1);
        assert!(p1.tdeg() == 2);

        // Monomials
        let yz: Mon<C> = Mon::from(md4);
        let x2: Mon<C> = Mon::from(md1);
        let xy: Mon<C> = Mon::from(md2);
        let y2: Mon<C> = Mon::from(md3);
        let twelve: Mon<C> = Mon::one() * C::new(12, 1);
        let p2 = Poly::from((vec![x2, yz, y2, twelve], r.clone()));

        /*
            Substitution Test
        */
        println!("{:?} subs {:?} to {:?} ", p1, x, p2);
        println!("{:?}", p1.subs(x, p2));

        /*
            Parametrized Reminder Test
        */
    }
}
