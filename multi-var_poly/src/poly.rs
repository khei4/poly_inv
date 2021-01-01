use super::coef::*;
use super::mon::*;
use super::ring::*;
use std::cell::RefCell;
use std::cmp::Reverse;
use std::rc::Rc;

#[derive(PartialEq, Eq, Clone)]
pub struct Poly {
    pub mons: Vec<Reverse<Mon<C>>>,
    pub r: Rc<RefCell<Ring>>,
}

// display, debug
impl std::fmt::Debug for Poly {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        assert!(0 < self.mons.len());
        let mut res = format!("{:?}", self.mons[0].0);
        for i in 1..self.mons.len() {
            if self.mons[i].0.coef > C::zero() {
                res = format!("{}+{:?}", res, self.mons[i].0);
            } else {
                res = format!("{}{:?}", res, self.mons[i].0);
            }
        }
        write!(f, "{}", res)
    }
}

// constructors
impl Poly {
    pub fn one(r: &Rc<RefCell<Ring>>) -> Poly {
        Poly::from((vec![Mon::<C>::one()], r))
    }
    pub fn zero(r: &Rc<RefCell<Ring>>) -> Poly {
        Poly::from((vec![Mon::<C>::zero()], r))
    }
}

#[test]
fn test_zero() {
    let r = Ring::new(vec![]);
    println!("{:?}", Poly::zero(&r));
}

impl From<(Vec<Mon<C>>, &Rc<RefCell<Ring>>)> for Poly {
    fn from(a: (Vec<Mon<C>>, &Rc<RefCell<Ring>>)) -> Self {
        let mut mons = vec![];
        for m in a.0 {
            mons.push(Reverse(m));
        }
        let mut p = Poly {
            mons,
            r: a.1.clone(),
        };
        p.sort_sumup();
        p
    }
}

// methods

impl Poly {
    pub fn is_zero(&self) -> bool {
        self.mons[0].0 == Mon::zero()
    }
    fn sort_sumup(&mut self) {
        // dummy monomial
        let dm = Reverse(Mon::<C>::zero());
        // 0を追加して, 最後にまとめて消す
        self.mons.sort();
        for i in 1..self.mons.len() {
            if !(self.mons[i - 1] > self.mons[i]) && !(self.mons[i - 1] < self.mons[i]) {
                let c = self.mons[i].0.coef;
                self.mons[i - 1].0.coef += c;
                self.mons[i] = dm.clone();
                if self.mons[i - 1].0.coef == C::zero() {
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
    pub fn tdeg(&self) -> usize {
        let m = self.mons.iter().min().expect("Poly T-degree Panic");
        m.0.vars.iter().fold(0, |s, (_, v)| s + v)
    }

    pub fn pow(&self, mut e: usize) -> Poly {
        let mut base = self.clone();
        let mut res = Poly::from((vec![Mon::one()], &self.r));
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
    // 0 -> x, 1 -> y, 2 -> z
    let x = Var::new(0);
    let y = Var::new(1);
    let z = Var::new(2);
    let vars = vec![x, y, z];
    let r = Ring::new(vars);

    // Monomials, Polynomials
    let x2: Mon<C> = Mon::from(vec![(x, 2)]);
    let xy: Mon<C> = Mon::from(vec![(x, 1), (y, 1)]);
    let yz: Mon<C> = Mon::from(vec![(y, 1), (z, 1)]);
    let p1 = Poly::from((vec![x2], &r));
    println!("{:?}", p1.pow(5));
}

impl std::ops::Neg for Poly {
    type Output = Poly;
    fn neg(mut self) -> Poly {
        for m in &mut self.mons {
            m.0.coef *= -C::one();
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
    // 0 -> x, 1 -> y, 2 -> z
    let x = Var::new(0);
    let y = Var::new(1);
    let z = Var::new(2);
    let vars = vec![x, y, z];
    let r = Ring::new(vars);

    let x2: Mon<C> = Mon::from(vec![(x, 2)]);
    let xy: Mon<C> = Mon::from(vec![(x, 1), (y, 1)]);
    let y2: Mon<C> = Mon::from(vec![(y, 2)]);
    let yz: Mon<C> = Mon::from(vec![(y, 1), (z, 1)]);
    let twelve: Mon<C> = Mon::one() * C::new(12, 1);
    let p1 = Poly::from((vec![x2, yz, twelve.clone()], &r));
    let p2 = Poly::from((vec![xy, y2, twelve], &r));
    let p3 = Poly::from((vec![], &r));
    assert!(p1.tdeg() == 2);
    assert!(p2.tdeg() == 2);
    let a = p1 + p2;
    assert!(a.tdeg() == 2);
    println!("{:?}", a);
}
