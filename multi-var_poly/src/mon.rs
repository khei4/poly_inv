/*
Variables
*/
// mod coef;

use super::coef::*;
use super::ring::*;

/*
Monomials
*/

use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[derive(PartialEq, Clone)]
pub struct Mon<T: Coef> {
    // Var and Deg
    pub vars: HashMap<Var, usize>,
    pub coef: T,
}

impl<T: Coef> Hash for Mon<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.coef.hash(state);
        let mut v: Vec<(Var, usize)> = self.vars.clone().into_iter().collect();
        v.sort();
        v.hash(state);
    }
}

impl<T: Coef> std::fmt::Debug for Mon<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut res: String;
        if self.is_cnst() {
            res = format!("{:?}", self.coef);
        } else {
            // TODO: coefの-1の分岐
            if T::zero() <= self.coef && self.coef != T::one() {
                res = String::from(format!("{:?}", self.coef));
            } else if self.coef < T::zero() {
                res = String::from(format!("{:?}", self.coef));
            } else {
                res = String::new();
            }

            {
                let mut resv = vec![];
                for (v, d) in self.vars.iter() {
                    resv.push((v, d));
                }
                resv.sort();
                // TODO: Ringを参照したいけど,ここまでとどかない, 単項式にも環をもたせる？？？
                // もしくは変数にStringをもたせちゃうか, Cloneはできなくなるけど
                for (v, d) in resv {
                    if *d != 1 {
                        res = format!("{}{:?}^{}", res, v, d);
                    } else {
                        res = format!("{}{:?}", res, v);
                    }
                }
            }
        }
        write!(f, "{}", res)
    }
}

impl<T: Coef> Mon<T> {
    // constantは, 変数
    pub fn one() -> Mon<T> {
        Mon {
            vars: HashMap::new(),
            coef: T::one(),
        }
    }

    // 番兵用/ zero
    pub fn zero() -> Mon<T> {
        Mon {
            vars: HashMap::new(),
            coef: T::zero(),
        }
    }

    pub fn is_cnst(&self) -> bool {
        self.vars.len() == 0
    }
}

impl<T: Coef> From<Var> for Mon<T> {
    fn from(v: Var) -> Self {
        let mut m = HashMap::new();
        m.insert(v, 1);
        Mon {
            vars: m,
            coef: T::one(),
        }
    }
}

impl<T: Coef> From<Vec<(Var, usize)>> for Mon<T> {
    fn from(v: Vec<(Var, usize)>) -> Self {
        Mon {
            vars: v.into_iter().collect(),
            coef: T::one(),
        }
    }
}

impl<T: Coef> From<HashMap<Var, usize>> for Mon<T> {
    fn from(m: HashMap<Var, usize>) -> Self {
        Mon {
            vars: m,
            coef: T::one(),
        }
    }
}

impl From<(Par, HashMap<Var, usize>)> for Mon<LinExp> {
    fn from(pm: (Par, HashMap<Var, usize>)) -> Self {
        Mon {
            vars: pm.1,
            coef: LinExp::from(pm.0),
        }
    }
}

impl From<(Par, Vec<(Var, usize)>)> for Mon<LinExp> {
    fn from(pm: (Par, Vec<(Var, usize)>)) -> Self {
        Mon {
            vars: pm.1.into_iter().collect(),
            coef: LinExp::from(pm.0),
        }
    }
}

impl<T: Coef> std::ops::Mul<Mon<C>> for Mon<T> {
    type Output = Mon<T>;
    fn mul(mut self, rhs: Mon<C>) -> Self::Output {
        let mut n: Mon<T> = Mon::one();
        // if LinExp multiplied, program crushes
        n.coef = self.coef * rhs.coef;
        for (v, d) in rhs.vars {
            match self.vars.get_mut(&v) {
                Some(d0) => *d0 += d,
                None => {
                    self.vars.insert(v, d);
                }
            }
        }
        n.vars = self.vars;
        n
    }
}

impl<T: Coef> std::ops::MulAssign<Mon<C>> for Mon<T> {
    fn mul_assign(&mut self, rhs: Mon<C>) {
        *self = self.clone() * rhs;
    }
}

impl<T: Coef> std::ops::Mul<C> for Mon<T> {
    type Output = Mon<T>;
    fn mul(mut self, rhs: C) -> Self::Output {
        self.coef *= rhs;
        if self.coef == T::zero() {
            self = Mon::zero();
        }
        self
    }
}

impl<T: Coef> std::ops::MulAssign<C> for Mon<T> {
    fn mul_assign(&mut self, rhs: C) {
        *self = self.clone() * rhs;
    }
}

impl<T: Coef> Eq for Mon<T> {}
impl<T: Coef> std::cmp::PartialOrd for Mon<T> {
    fn partial_cmp(&self, rhs: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(rhs))
    }
}
// Grevlex
impl<T: Coef> std::cmp::Ord for Mon<T> {
    fn cmp(&self, rhs: &Self) -> std::cmp::Ordering {
        if self.coef == T::zero() {
            return std::cmp::Ordering::Less;
        } else if rhs.coef == T::zero() {
            return std::cmp::Ordering::Greater;
        }
        let lmd: usize = self.vars.iter().fold(0, |s, (_, d)| s + d);
        let rmd: usize = rhs.vars.iter().fold(0, |s, (_, d)| s + d);
        if lmd != rmd {
            lmd.cmp(&rmd)
        } else {
            // ここがひどい

            // 変数の一覧を取得
            let mut m = self.vars.clone();
            m.extend(rhs.vars.clone());
            let mut diff: HashMap<Var, usize> = HashMap::new();
            for k in m.keys() {
                diff.insert(*k, 0);
            }
            for (v, d) in self.vars.iter() {
                diff.insert(*v, *d);
            }
            let mut diff_vec = vec![];
            for (v, d) in diff {
                match rhs.vars.get(&v) {
                    Some(dv) => diff_vec.push((v, *dv as i64 - d as i64)),
                    None => {
                        diff_vec.push((v, -(d as i64)));
                    }
                }
            }
            diff_vec.sort();
            for i in (0..diff_vec.len()).rev() {
                if diff_vec[i].1 < 0 {
                    // self
                    return std::cmp::Ordering::Less;
                } else if diff_vec[i].1 > 0 {
                    // rhs
                    return std::cmp::Ordering::Greater;
                }
            }
            return std::cmp::Ordering::Equal;
        }
    }
}

#[test]
fn mon_ord_test() {
    // Init Ring
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
    let eight = Mon::one() * C::new(8, 1);
    let z = Mon::zero();
    assert!(z < eight);
    assert!(xy < x2);
    let mut dp = vec![x2, xy, y2, yz, eight, z];
    dp.sort();
    println!("{:?}", dp);
}
