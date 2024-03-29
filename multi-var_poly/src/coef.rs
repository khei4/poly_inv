/*
Parameters
*/

/*
Parameter Terms
*/

use super::ring::*;
// BigIntにすると, ParTermがCopyじゃなくなる
pub use num_rational::Rational64;
pub use num_traits::identities::{One, Zero};
use std::hash::Hash;
pub type C = Rational64;
#[derive(Clone, Copy, PartialEq, Hash)]
pub struct ParTerm {
    pub par: Option<Par>,
    pub coef: C,
}

impl std::fmt::Debug for ParTerm {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let res;
        if self.is_cnst() {
            res = format!("{}", self.coef);
        } else if self.coef == C::one() {
            res = format!("{:?}", self.par.expect("par debug failed"));
        } else if self.coef == -C::one() {
            res = format!("-{:?}", self.par.expect("par debug failed"));
        } else {
            res = format!("{}{:?}", self.coef, self.par.expect("par debug failed"));
        }
        write!(f, "{}", res)
    }
}

impl Eq for ParTerm {}

impl ParTerm {
    pub fn zero() -> Self {
        ParTerm {
            par: None,
            coef: C::zero(),
        }
    }
    pub fn is_zero(self) -> bool {
        self == ParTerm::zero()
    }
    fn one() -> Self {
        ParTerm {
            par: None,
            coef: C::one(),
        }
    }
    pub fn is_cnst(self) -> bool {
        self.par.is_none()
    }
}

impl From<Par> for ParTerm {
    fn from(par: Par) -> Self {
        ParTerm {
            par: Some(par),
            coef: C::one(),
        }
    }
}

impl std::cmp::PartialOrd for ParTerm {
    fn partial_cmp(&self, rhs: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(rhs))
    }
}
// 辞書式 かつ, 0が最小
impl std::cmp::Ord for ParTerm {
    fn cmp(&self, rhs: &Self) -> std::cmp::Ordering {
        if self.is_cnst() && rhs.is_cnst() {
            if self.is_zero() {
                std::cmp::Ordering::Less
            } else if rhs.is_zero() {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        } else {
            self.par.cmp(&rhs.par)
        }
    }
}

impl std::ops::Mul<C> for ParTerm {
    type Output = ParTerm;

    fn mul(mut self, other: C) -> Self::Output {
        self.coef *= other;
        self
    }
}

impl std::ops::MulAssign<C> for ParTerm {
    fn mul_assign(&mut self, rhs: C) {
        *self = self.clone() * rhs;
    }
}

#[test]
fn parterm_ord_test() {
    // ParTermのオーダーは辞書順
    // var is lexicographic
    let a = ParTerm::from(Par::new(0));
    let c = ParTerm::from(Par::new(2));
    let a = a * C::new(8, 1);
    assert!(a < c);

    // cnst < var
    let z = ParTerm::zero();
    assert!(z < a);
    let o = ParTerm::one();
    assert!(o < a);
    // zero is minimum
    assert!(z < o);
}

/*
Linear Expressions of Parameter (by Vec)
*/

#[derive(Clone, PartialEq, Eq, PartialOrd, Hash)]
pub struct LinExp {
    pub terms: Vec<ParTerm>,
}

impl std::fmt::Debug for LinExp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        assert!(0 < self.terms.len());
        let mut res = format!("{:?}", self.terms[0]);
        if !self.is_cnst() {
            for i in 1..self.terms.len() {
                if self.terms[i].coef > C::zero() {
                    res = format!("{}+{:?}", res, self.terms[i]);
                } else {
                    res = format!("{}{:?}", res, self.terms[i]);
                }
            }
        }
        write!(f, "({})", res)
    }
}

impl LinExp {
    fn sort_sumup(&mut self) {
        // 0を追加して, 最後にまとめて消す
        let z = ParTerm::zero();
        self.terms.sort_by(|x, y| y.cmp(&x));
        for i in 1..self.terms.len() {
            if !(self.terms[i - 1] > self.terms[i]) && !(self.terms[i - 1] < self.terms[i]) {
                let c = self.terms[i].coef;
                self.terms[i - 1].coef += c;
                self.terms[i] = z;
                if self.terms[i - 1].coef == C::zero() {
                    self.terms[i - 1] = z;
                }
            }
        }
        self.terms.sort_by(|x, y| y.cmp(&x));
        while let Some(m) = self.terms.pop() {
            if m != z {
                self.terms.push(m);
                break;
            }
        }
        if self.terms.len() == 0 {
            self.terms.push(z);
        }
    }
    pub fn is_cnst(&self) -> bool {
        assert!(0 < self.terms.len());
        self.terms.len() == 1 && self.terms[0].par.is_none()
    }
}

impl One for LinExp {
    fn one() -> LinExp {
        LinExp {
            terms: vec![ParTerm::one()],
        }
    }
}

impl Zero for LinExp {
    fn zero() -> LinExp {
        LinExp {
            terms: vec![ParTerm::zero()],
        }
    }

    fn is_zero(&self) -> bool {
        self.terms.len() == 1 && self.terms[0] == ParTerm::zero()
    }
}
impl From<Vec<ParTerm>> for LinExp {
    fn from(mut terms: Vec<ParTerm>) -> Self {
        terms.sort();
        if terms.len() == 0 {
            terms = vec![ParTerm::zero()];
        }
        LinExp { terms }
    }
}
impl From<Par> for LinExp {
    fn from(p: Par) -> Self {
        LinExp {
            terms: vec![ParTerm::from(p)],
        }
    }
}

impl std::ops::Add<LinExp> for LinExp {
    type Output = LinExp;

    fn add(mut self, other: LinExp) -> LinExp {
        // 結合して, ソートする
        let z = ParTerm::zero();
        self.terms.extend(other.terms);
        self.terms.sort_by(|x, y| y.cmp(&x));
        for i in 1..self.terms.len() {
            if self.terms[i - 1] <= self.terms[i] && self.terms[i] <= self.terms[i - 1] {
                let c = self.terms[i].coef;
                self.terms[i - 1].coef += c;
                if self.terms[i - 1].coef == C::zero() {
                    self.terms[i - 1] = z;
                }
                self.terms[i] = z;
            }
        }
        self.terms.sort_by(|x, y| y.cmp(&x));
        while let Some(m) = self.terms.pop() {
            if m != z {
                self.terms.push(m);
                break;
            }
        }
        self.terms.sort();
        if self.terms.len() == 0 {
            self.terms.push(z);
        }
        self
    }
}
impl std::ops::AddAssign<LinExp> for LinExp {
    fn add_assign(&mut self, rhs: LinExp) {
        *self = self.clone() + rhs;
    }
}

impl std::ops::Mul<LinExp> for LinExp {
    type Output = LinExp;
    fn mul(self, _other: LinExp) -> Self::Output {
        unreachable!();
        // self
    }
}

impl std::ops::Add<C> for LinExp {
    type Output = LinExp;

    fn add(mut self, other: C) -> Self::Output {
        let a = ParTerm::one() * other;
        if let Some(l) = self.terms.last_mut() {
            if *l >= a {
                l.coef += other;
                if l.coef == C::zero() {
                    self.terms.pop();
                }
            } else {
                self.terms.push(a);
            }
        }
        self
    }
}

impl std::ops::AddAssign<C> for LinExp {
    fn add_assign(&mut self, rhs: C) {
        *self = self.clone() + rhs;
    }
}
impl std::ops::Mul<C> for LinExp {
    type Output = LinExp;

    fn mul(mut self, other: C) -> Self::Output {
        if other == C::zero() {
            LinExp::zero()
        } else {
            for t in &mut self.terms {
                *t *= other;
            }
            self
        }
    }
}
impl std::ops::MulAssign<C> for LinExp {
    fn mul_assign(&mut self, rhs: C) {
        *self = self.clone() * rhs;
    }
}

impl std::ops::Neg for LinExp {
    type Output = LinExp;
    fn neg(mut self) -> LinExp {
        for t in &mut self.terms {
            t.coef *= -C::one();
        }
        self
    }
}

#[test]
fn linexp_ops_test() {
    let threea = ParTerm::from(Par::new(0)) * C::new(3, 1);
    let twob = ParTerm::from(Par::new(1));
    let onec = ParTerm::from(Par::new(2));
    let le1 = LinExp::from(vec![threea, twob * C::new(-1, 1), onec]);
    let le2 = LinExp::from(vec![twob, onec]);
    // TODO:
    println!("{:?}", le1);
    println!("{:?}", le1.clone() * C::new(9, 1));
    println!("{:?}", le1.clone() * C::zero());
    let les = le1 + le2;
    println!("{:?}", les);

    // zero and one
    assert!(LinExp::zero() + les.clone() == les.clone());
    assert!(les.clone() + LinExp::zero() == les.clone());
    assert!(LinExp::one() + LinExp::zero() == LinExp::one());
}

/*

*/

pub trait Coef:
    Clone
    + std::cmp::PartialEq
    + std::cmp::PartialOrd
    + std::ops::Add<Self, Output = Self>
    + std::ops::Add<C, Output = Self>
    + std::ops::AddAssign<Self>
    + std::ops::AddAssign<C>
    + std::ops::Neg<Output = Self>
    + std::ops::Mul<Self, Output = Self>
    + std::ops::Mul<C, Output = Self>
    + std::ops::MulAssign<C>
    + std::fmt::Debug
    + One
    + Zero
    + Hash
{
}

impl Coef for LinExp {}

impl Coef for C {}
