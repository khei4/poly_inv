use super::mon::*;
use super::ring::*;
use std::cell::RefCell;
use std::cmp::Reverse;
use std::rc::Rc;

#[derive(PartialEq, Clone)]
pub struct Poly {
    pub mons: Vec<Reverse<Mon<f64>>>,
    pub r: Rc<RefCell<Ring>>,
}

// display, debug
impl std::fmt::Debug for Poly {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut res = format!("{:?}", self.mons[0].0);
        for i in 1..self.mons.len() {
            if self.mons[i].0.coef > 0. {
                res = format!("{}+{:?}", res, self.mons[i].0);
            } else {
                res = format!("{}{:?}", res, self.mons[i].0);
            }
        }
        write!(f, "{}", res)
    }
}

impl From<(Vec<Mon<f64>>, Rc<RefCell<Ring>>)> for Poly {
    fn from(a: (Vec<Mon<f64>>, Rc<RefCell<Ring>>)) -> Self {
        let mut mons = vec![];
        for m in a.0 {
            mons.push(Reverse(m));
        }
        let mut p = Poly { mons, r: a.1 };
        p.sort_sumup();
        p
    }
}

// methods

impl Poly {
    pub fn one(r: Rc<RefCell<Ring>>) -> Poly {
        Poly::from((vec![Mon::<f64>::one()], r))
    }
    fn sort_sumup(&mut self) {
        // dummy monomial
        let dm = Reverse(Mon::<f64>::zero());
        // 0を追加して, 最後にまとめて消す
        self.mons.sort();
        for i in 1..self.mons.len() {
            if !(self.mons[i - 1] > self.mons[i]) && !(self.mons[i - 1] < self.mons[i]) {
                self.mons[i - 1].0.coef += self.mons[i].0.coef;
                self.mons[i] = dm.clone();
                if self.mons[i - 1].0.coef == 0. {
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
    pub fn tdeg(&self) -> usize {
        self.mons[0].0.vars.iter().fold(0, |s, (_, v)| s + v)
    }

    pub fn pow(&self, mut e: usize) -> Poly {
        let mut base = self.clone();
        let mut res = Poly::from((vec![Mon::one()], self.r.clone()));
        while e > 0 {
            if e & 1 == 1 {
                res *= base.clone();
            }
            base *= base.clone();
            e >>= 1;
        }
        res
    }
}
#[test]
fn check_poly_pow() {
    use std::collections::HashMap;
    let x: Var = Var::new('x');
    let y = Var::new('y');
    let z = Var::new('z');
    let vars = vec![x, y, z];
    let r = Rc::new(RefCell::new(Ring::from(vars)));

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

    let yz: Mon<f64> = Mon::from(md4);
    let x2: Mon<f64> = Mon::from(md1);
    let xy: Mon<f64> = Mon::from(md2);
    let y2: Mon<f64> = Mon::from(md3);
    assert!(xy > yz);
    let one: Mon<f64> = Mon::one();
    let p1 = Poly::from((vec![x2], r.clone()));
    println!("{:?}", p1.pow(5));
}

impl std::ops::Neg for Poly {
    type Output = Poly;
    fn neg(mut self) -> Poly {
        for m in &mut self.mons {
            m.0.coef *= -1.;
        }
        self
    }
}

impl std::ops::Add<Poly> for Poly {
    type Output = Poly;

    fn add(mut self, rhs: Poly) -> Self::Output {
        self.mons.extend(rhs.mons);
        self.sort_sumup();
        self
    }
}

impl std::ops::AddAssign<Poly> for Poly {
    fn add_assign(&mut self, rhs: Poly) {
        *self = self.clone() + rhs;
    }
}

// TODO: O(N^2)ですが
impl std::ops::Mul<Poly> for Poly {
    type Output = Poly;

    fn mul(mut self, rhs: Poly) -> Self::Output {
        let mut tmp = vec![];
        for m1 in &self.mons {
            for m2 in &rhs.mons {
                tmp.push(Reverse(m1.0.clone() * m2.0.clone()));
            }
        }
        self.mons = tmp;
        self.sort_sumup();
        self
    }
}

impl std::ops::MulAssign<Poly> for Poly {
    fn mul_assign(&mut self, rhs: Poly) {
        *self = self.clone() * rhs;
    }
}

#[test]
fn check_poly_addition() {
    use std::collections::HashMap;
    let x: Var = Var::new('x');
    let y = Var::new('y');
    let z = Var::new('z');
    let vars = vec![x, y, z];
    let r = Rc::new(RefCell::new(Ring::from(vars)));

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

    let yz: Mon<f64> = Mon::from(md4);
    let x2: Mon<f64> = Mon::from(md1);
    let xy: Mon<f64> = Mon::from(md2);
    let y2: Mon<f64> = Mon::from(md3);
    assert!(xy > yz);
    let one: Mon<f64> = Mon::one();
    let p1 = Poly::from((vec![x2, yz, one.clone() * 12.], r.clone()));
    let p2 = Poly::from((vec![xy, y2, one * 9.], r.clone()));
    let p3 = Poly::from((vec![], r.clone()));
    println!("{:?}", p1.r);
    p3.r.borrow_mut().pextend(vec![Par::new(0)]);
    println!("{:?}", p1.r);
    assert!(p1.tdeg() == 2);
    assert!(p2.tdeg() == 2);
    let mut a = p1 + p2;
    assert!(a.tdeg() == 2);
    println!("{:?}", a);
}
