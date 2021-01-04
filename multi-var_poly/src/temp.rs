use super::coef::*;
use super::mon::*;
use super::poly::*;
use super::ring::*;
use itertools::Itertools;
use std::cell::RefCell;
use std::cmp::Reverse;
use std::collections::HashMap;
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

impl std::cmp::PartialOrd for Temp {
    fn partial_cmp(&self, other: &Temp) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::Ord for Temp {
    fn cmp(&self, other: &Temp) -> std::cmp::Ordering {
        other
            .mons
            .iter()
            .min()
            .expect("Temp T-degree Panic")
            .cmp(&self.mons.iter().min().expect("Temp T-degree Panic"))
    }
}

impl std::fmt::Debug for Temp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut res = format!("{}", self.mons[0].0);
        for i in 1..self.mons.len() {
            if self.mons[i].0.coef >= LinExp::zero() {
                res = format!("{}+{}", res, self.mons[i].0);
            } else {
                res = format!("{}-{}", res, self.mons[i].0);
            }
        }
        write!(f, "{}", res)
    }
}

// constructers
impl Temp {
    pub fn zero(r: &Rc<RefCell<Ring>>) -> Temp {
        Temp {
            mons: vec![Reverse(Mon::zero(r))],
            r: r.clone(),
        }
    }
    fn one(r: &Rc<RefCell<Ring>>) -> Temp {
        Temp {
            mons: vec![Reverse(Mon::one(r))],
            r: r.clone(),
        }
    }
}

#[test]
fn zero_is_identity_of_add() {
    let r = Ring::new();
    assert!(Temp::one(&r) + Temp::zero(&r) == Temp::one(&r));
    assert!(Temp::zero(&r) + Temp::one(&r) == Temp::one(&r));
    assert!(Temp::zero(&r) + Temp::zero(&r) == Temp::zero(&r));
}

// construct Temp with extend Rings
impl From<(Vec<Mon<LinExp>>, &Rc<RefCell<Ring>>)> for Temp {
    fn from(a: (Vec<Mon<LinExp>>, &Rc<RefCell<Ring>>)) -> Self {
        let (a, r) = a;
        let mut mons = vec![];
        for m in a {
            for pt in &m.coef.terms {
                match pt.par {
                    Some(p) => drop(r.borrow_mut().pars.insert(p)),
                    None => (),
                }
            }
            mons.push(Reverse(m));
        }
        if mons.len() == 0 {
            mons.push(Reverse(Mon::zero(r)));
        }
        let mut p = Temp { mons, r: r.clone() };
        p.sort_sumup();
        p
    }
}

// methods
impl Temp {
    fn is_zero(&self) -> bool {
        self.mons[0].0 == Mon::zero(&self.r)
    }
    fn sort_sumup(&mut self) {
        // dummy monomial
        let dm = Reverse(Mon::<LinExp>::zero(&self.r));
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

    pub fn most_gen(d: usize, r: &Rc<RefCell<Ring>>) -> Temp {
        let v: Vec<Var> = r
            .borrow_mut()
            .vars
            .clone()
            .into_iter()
            .map(|(v, _s)| v)
            .collect();
        let mut cnt = r.borrow().pars.len();
        let mut dummy_poly = Poly::one(r);
        for (v, _s) in &r.borrow().vars {
            dummy_poly += Poly::from((*v, r));
        }
        dummy_poly = dummy_poly.pow(d);
        let mut res = Temp::zero(r);
        let mut fresh_pars = vec![];
        // これをすると, 定数項からIndexがついていく
        while let Some(Reverse(m)) = dummy_poly.mons.pop() {
            let fp = Par::new(cnt);
            cnt += 1;
            let new_m = Mon::from((fp, m.vars, r));
            fresh_pars.push(fp);
            res.mons.push(Reverse(new_m));
        }
        // ソートは別にしなくていいんだけどね(popじゃなくて順番を管理すればよい)
        r.borrow_mut().pextend(fresh_pars);
        res.sort_sumup();
        res
    }
    // randomized version
    // pub fn most_gen(d: usize, r: &Rc<RefCell<Ring>>) -> Temp {
    //     let v: Vec<Var> = r
    //         .borrow()
    //         .vars
    //         .clone()
    //         .into_iter()
    //         .map(|(v, _s)| v)
    //         .collect();
    //     let mut cnt = r.borrow().pars.len();
    //     let mut fresh_pars = vec![];
    //     let mut mons = vec![];
    //     for i in 0..d + 1 {
    //         for c in v.iter().combinations_with_replacement(i) {
    //             // これはケッコーやばい
    //             let fp = Par::new(cnt);
    //             cnt += 1;
    //             fresh_pars.push(fp);
    //             let mut m: std::collections::HashMap<Var, usize> = std::collections::HashMap::new();
    //             for v in c {
    //                 match m.get_mut(&v) {
    //                     Some(d) => *d += 1,
    //                     None => {
    //                         m.insert(*v, 1);
    //                     }
    //                 }
    //             }
    //             mons.push(Reverse(Mon::<LinExp>::from((fp, m, r))))
    //         }
    //     }
    //     r.borrow_mut().pextend(fresh_pars);
    //     mons.sort();
    //     Temp { mons, r: r.clone() }
    // }

    pub fn rem_par(&self, other: Poly) -> Temp {
        let diff = self.tdeg() - other.tdeg();
        let q = Temp::most_gen(diff, &self.r);
        q * (-other) + self.clone()
    }

    pub fn subs(mut self, v: Var, other: Poly) -> Temp {
        self.sort_by_var(v);
        let mut res = Temp::zero(&self.r);
        let mut base = Poly::one(&self.r);
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
            res += Temp::from((vec![m.0.clone()], &self.r)) * base.clone();
        }
        res
    }

    pub fn subs_pars(&self, sol: Vec<(Par, LinExp)>) -> Temp {
        let sol_map = sol.into_iter().collect::<HashMap<Par, LinExp>>();
        // 各単項式の
        let mut res_mons: Vec<Mon<LinExp>> = vec![];
        for Reverse(m) in &self.mons {
            for pt in &m.coef.terms {
                let new_linexp;
                match pt.par {
                    Some(p) => new_linexp = sol_map[&p].clone() * pt.coef,
                    None => new_linexp = LinExp::one() * pt.coef,
                }
                if new_linexp.is_zero() {
                    continue;
                }
                let mut new_mon = Mon::from((m.vars.clone(), &self.r));
                new_mon.coef = new_linexp;
                res_mons.push(new_mon);
            }
        }
        // こうやって作ると, 新しいパラメーターでRingが拡大されてしまうけど...
        Temp::from((res_mons, &self.r))
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
            Temp::zero(&self.r)
        } else {
            let mut new_terms: Vec<Reverse<Mon<LinExp>>> = vec![];
            for m in &other.mons {
                for tm in &self.mons {
                    new_terms.push(Reverse(tm.0.clone() * m.0.clone()))
                }
            }
            if new_terms.len() == 0 {
                Temp::zero(&self.r)
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
    let r = Ring::new();
    assert!(Temp::one(&r) * Poly::zero(&r) == Temp::zero(&r));
    println!("T 1 * P 0 = T 0");
    assert!(Temp::zero(&r) * Poly::one(&r) == Temp::zero(&r));
    println!("T 0 * P 1 = T 0");
    assert!(Temp::one(&r) * Poly::one(&r) == Temp::one(&r));
    println!("T 1 * P 1 = T 1");
    assert!(Temp::zero(&r) * Poly::zero(&r) == Temp::zero(&r));
    println!("T 0 * P 0 = T 0");
}

impl std::ops::MulAssign<Poly> for Temp {
    fn mul_assign(&mut self, other: Poly) {
        *self = self.clone() * other;
    }
}

impl std::ops::Neg for Temp {
    type Output = Temp;
    fn neg(mut self) -> Temp {
        for m in &mut self.mons {
            m.0.coef *= -C::one();
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_temp_add_poly_mul() {
        // Init Ring
        // 0 -> x, 1 -> y, 2 -> z
        let r = Ring::new();
        let x = r.borrow_mut().vextend("x".to_string());
        let y = r.borrow_mut().vextend("y".to_string());
        let z = r.borrow_mut().vextend("z".to_string());
        // parameters
        let pars: Vec<Par> = (0..4).map(|i| Par::new(i)).collect();
        // Init Template

        let ax2: Mon<LinExp> = Mon::from((pars[0], vec![(x, 2)], &r));
        let bx2: Mon<LinExp> = Mon::from((pars[1], vec![(x, 1), (y, 1)], &r));
        let cxy: Mon<LinExp> = Mon::from((pars[2], vec![(x, 1), (y, 1)], &r));
        let dxy: Mon<LinExp> = Mon::from((pars[3], vec![(x, 1), (y, 1)], &r));
        let y2: Mon<LinExp> = Mon::from((pars[3], vec![(y, 2)], &r));
        let yz: Mon<LinExp> = Mon::from((pars[0], vec![(y, 1), (z, 1)], &r));
        let p1 = Temp::from((vec![ax2, cxy, yz, y2.clone()], &r));
        let p2 = Temp::from((vec![bx2, dxy, y2], &r));
        // もれなくだぶりなく拡張されている
        assert!(r.as_ref().borrow().pars == pars.into_iter().collect());
        assert!(p1.tdeg() == 2);
        assert!(p2.tdeg() == 2);

        /*
            Addition Test
        */

        let p1 = p1 + p2;

        assert!(p1.clone() == p1.clone() + Temp::zero(&r));
        assert!(p1.clone() == Temp::zero(&r) + p1.clone());

        // Monomials, Polynomials
        let x2: Mon<C> = Mon::from((vec![(x, 2)], &r));
        let y2: Mon<C> = Mon::from((vec![(y, 2)], &r));
        let xy: Mon<C> = Mon::from((vec![(x, 1), (y, 1)], &r));
        let yz: Mon<C> = Mon::from((vec![(y, 1), (z, 1)], &r));
        let twelve: Mon<C> = Mon::one(&r) * C::new(12, 1);
        let p2 = Poly::from((vec![x2, yz, xy, y2, twelve], &r));

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
        // 0 -> x, 1 -> y, 2 -> z
        let r = Ring::new();
        let x = r.borrow_mut().vextend("x".to_string());
        let y = r.borrow_mut().vextend("y".to_string());
        let z = r.borrow_mut().vextend("z".to_string());

        // Init Template by
        /*
            Most Generic Template
        */
        let p1 = Temp::most_gen(2, &r);
        // 3 variable, 2 degree => 4H2 == 5C2 == 10
        assert!(r.as_ref().borrow().pars.len() == 10);
        println!("{:?}", p1);
        assert!(p1.tdeg() == 2);

        // Monomials
        let x2: Mon<C> = Mon::from((vec![(x, 2)], &r));
        let y2: Mon<C> = Mon::from((vec![(y, 2)], &r));
        let xy: Mon<C> = Mon::from((vec![(x, 1), (y, 1)], &r));
        let yz: Mon<C> = Mon::from((vec![(y, 1), (z, 1)], &r));
        let twelve: Mon<C> = Mon::one(&r) * C::new(12, 1);
        let p2 = Poly::from((vec![x2, yz, y2, twelve], &r));

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
        // 0 -> x1, 1 -> x2, 2 -> y1, 3 -> y2, 4 -> y3
        let r = Ring::new();
        let x1 = r.borrow_mut().vextend("x1".to_string());
        let x2 = r.borrow_mut().vextend("x2".to_string());
        let y1 = r.borrow_mut().vextend("y1".to_string());
        let y2 = r.borrow_mut().vextend("y2".to_string());
        let y3 = r.borrow_mut().vextend("y3".to_string());
        // Init Invariant (Template)
        // y1*x2 + y2 + y3 - x1 = 0

        let m1: Mon<LinExp> = Mon::from((vec![(y1, 1), (x2, 1)], &r));
        let m2: Mon<LinExp> = Mon::from((vec![(y2, 1)], &r));
        let m3: Mon<LinExp> = Mon::from((vec![(y3, 1)], &r));
        let m4: Mon<LinExp> = Mon::from((vec![(x1, 1)], &r)) * -C::one();
        let g_inv = Temp::from((vec![m1, m2, m3, m4], &r));

        // guard polynomial
        // p = x2-y2-1
        let m1: Mon<C> = Mon::from((vec![(x2, 1)], &r));
        let m2: Mon<C> = Mon::from((vec![(y2, 1)], &r)) * -C::one();
        let n_one: Mon<C> = Mon::one(&r) * -C::one();
        let p = Poly::from((vec![m1, m2, n_one], &r));

        // subs poly pcxyVn => cx's y-th substitution to Vn variable
        let pc11y1 = Poly::from((vec![Mon::from((vec![(y1, 1)], &r)), Mon::one(&r)], &r));
        let pc12y2 = Poly::zero(&r);
        let pc13y3 = Poly::from((
            vec![Mon::from((vec![(y3, 1)], &r)), Mon::one(&r) * -C::one()],
            &r,
        ));
        let pc21y2 = Poly::from((vec![Mon::from((vec![(y2, 1)], &r)), Mon::one(&r)], &r));
        let pc22y3 = Poly::from((
            vec![Mon::from((vec![(y3, 1)], &r)), Mon::one(&r) * -C::one()],
            &r,
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
        let p1y1 = Poly::zero(&r);
        let p2y2 = Poly::zero(&r);
        let p3y3 = Poly::from((vec![Mon::from((vec![(x1, 1)], &r))], &r));
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
