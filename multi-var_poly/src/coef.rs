/*
Parameters
*/

// 辞書式
#[derive(Eq, PartialEq, PartialOrd, Ord, Clone, Copy, Hash)]
struct Par {
    sym: char,
}

impl Par {
    fn new(c: char) -> Par {
        Par { sym: c }
    }
}

impl std::fmt::Debug for Par {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.sym)
    }
}

// TODO: sortがガバガバぽい

/*
Parameter Terms
TODO: Debug trait
*/

#[derive(Clone, Copy, PartialEq)]
struct ParTerm {
    par: Option<Par>,
    coef: f64,
}
impl Eq for ParTerm {}

impl ParTerm {
    fn zero() -> Self {
        ParTerm {
            par: None,
            coef: 0.,
        }
    }
    fn one() -> Self {
        ParTerm {
            par: None,
            coef: 1.,
        }
    }
}

impl From<Par> for ParTerm {
    fn from(par: Par) -> Self {
        ParTerm {
            par: Some(par),
            coef: 1.,
        }
    }
}

impl std::cmp::PartialOrd for ParTerm {
    fn partial_cmp(&self, rhs: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(rhs))
    }
}
// 辞書式(の逆)
impl std::cmp::Ord for ParTerm {
    fn cmp(&self, rhs: &Self) -> std::cmp::Ordering {
        rhs.par.cmp(&self.par)
    }
}

impl std::ops::Mul<f64> for ParTerm {
    type Output = ParTerm;

    fn mul(mut self, other: f64) -> Self::Output {
        self.coef *= other;
        self
    }
}

impl std::ops::MulAssign<f64> for ParTerm {
    fn mul_assign(&mut self, rhs: f64) {
        *self = self.clone() * rhs;
    }
}

#[test]
fn parterm_ord_test() {
    let o = ParTerm::one();
    let z = ParTerm::zero();
    assert!(z <= o);
    assert!(o <= z);

    let a = ParTerm::from(Par::new('a'));
    let c = ParTerm::from(Par::new('c'));
    assert!(c < a);
    assert!(a < z);
}

/*
Linear Expressions of Parameter (by Vec)
TODO: Debug trait
*/

#[derive(Clone)]
struct LinExp {
    terms: Vec<ParTerm>,
}

impl From<Vec<ParTerm>> for LinExp {
    fn from(terms: Vec<ParTerm>) -> Self {
        LinExp { terms }
    }
}

impl std::ops::Add<LinExp> for LinExp {
    type Output = LinExp;

    fn add(mut self, other: LinExp) -> LinExp {
        // 結合して, ソートする
        let z = ParTerm::zero();
        self.terms.extend(other.terms);
        self.terms.sort();
        for i in 1..self.terms.len() {
            if self.terms[i - 1] <= self.terms[i] && self.terms[i] <= self.terms[i - 1] {
                self.terms[i - 1].coef += self.terms[i].coef;
                if self.terms[i - 1].coef == 0. {
                    self.terms[i - 1] = z;
                }
                self.terms[i] = z;
            }
        }
        self.terms.sort();
        while let Some(m) = self.terms.pop() {
            if m != z {
                self.terms.push(m);
                break;
            }
        }
        self
    }
}

impl std::ops::Add<f64> for LinExp {
    type Output = LinExp;

    fn add(mut self, other: f64) -> Self::Output {
        let a = ParTerm::one() * other;
        if let Some(l) = self.terms.last_mut() {
            if *l >= a {
                l.coef += other;
                if l.coef == 0. {
                    self.terms.pop();
                }
            } else {
                self.terms.push(a);
            }
        }
        self
    }
}

impl std::ops::AddAssign<f64> for LinExp {
    fn add_assign(&mut self, rhs: f64) {
        *self = self.clone() + rhs;
    }
}

impl std::ops::Mul<f64> for LinExp {
    type Output = LinExp;

    fn mul(mut self, other: f64) -> Self::Output {
        if other == 0. {
            LinExp::zero()
        } else {
            for t in &mut self.terms {
                *t *= other;
            }
            self
        }
    }
}
impl std::ops::MulAssign<f64> for LinExp {
    fn mul_assign(&mut self, rhs: f64) {
        *self = self.clone() * rhs;
    }
}
#[test]
fn linexp_ops_test() {
    let three = ParTerm::one() * 3.;

    let threea = ParTerm::from(Par::new('a')) * 3.;
    let onec = ParTerm::from(Par::new('c'));
    // LinExp::
    // assert!(c < a);
    // assert!(a < z);
}

/*

*/

pub trait Coef:
    Clone
    + std::ops::Add<Self, Output = Self>
    + std::ops::Add<f64, Output = Self>
    + std::ops::AddAssign<f64>
    + std::ops::Mul<f64, Output = Self>
    + std::ops::MulAssign<f64>
{
    fn zero() -> Self;
    fn one() -> Self;
}

impl Coef for LinExp {
    fn one() -> LinExp {
        LinExp {
            terms: vec![ParTerm::one()],
        }
    }
    fn zero() -> LinExp {
        LinExp {
            terms: vec![ParTerm::zero()],
        }
    }
}

impl Coef for f64 {
    fn zero() -> f64 {
        0.
    }

    fn one() -> f64 {
        1.
    }
}
