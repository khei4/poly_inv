/*
実係数(f64)多変数多項式
- 変数順序は辞書順にする (x < y < z < w)
- 単項式順序はgrevlex
- 掛け算はナイーブ
TODO:
*/

// =========

/*
Variables
*/
// mod coef;
use super::coef::*;

#[derive(Eq, PartialEq, PartialOrd, Ord, Clone, Copy, Hash)]
struct Var {
    sym: char,
}
impl Var {
    fn new(c: char) -> Var {
        Var { sym: c }
    }
}

impl std::fmt::Debug for Var {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.sym)
    }
}

/*
Monomials
*/

use std::collections::HashMap;

#[derive(PartialEq, Clone)]
pub struct Mon<T: Coef> {
    // Var and Deg
    vars: HashMap<Var, usize>,
    coef: T,
}

impl<T: Coef> std::fmt::Debug for Mon<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // constant
        let mut res: String;
        // 定数がなぜか 表示されないけどいいや
        if self.vars.get(&Var::new('1')).is_some() || self.vars.get(&Var::new('0')).is_some() {
            res = format!("{:?}", self.coef);
        } else {
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
                for (v, d) in resv {
                    res = format!("{}{}{}", res, v.sym, d);
                }
            }
        }
        write!(f, "{}", res)
    }
}

impl<T: Coef> Mon<T> {
    // constantは, 変数
    fn one() -> Mon<T> {
        Mon {
            vars: HashMap::new(),
            coef: T::one(),
        }
    }

    // 番兵用/ zero
    fn zero() -> Mon<T> {
        Mon {
            vars: HashMap::new(),
            coef: T::zero(),
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

impl<T: Coef> std::ops::Mul<Mon<T>> for Mon<T> {
    type Output = Mon<T>;
    fn mul(mut self, rhs: Mon<T>) -> Self::Output {
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
// scalar multiplication
impl<T: Coef> std::ops::Mul<f64> for Mon<T> {
    type Output = Mon<T>;
    fn mul(mut self, rhs: f64) -> Self::Output {
        self.coef *= rhs;
        if self.coef == T::zero() {
            self = Mon::zero();
        }
        self
    }
}

impl<T: Coef> std::ops::MulAssign<Mon<T>> for Mon<T> {
    fn mul_assign(&mut self, rhs: Mon<T>) {
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
    let x: Var = Var::new('x');
    let y = Var::new('y');
    let z = Var::new('z');

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

    let x2: Mon<f64> = Mon::from(md1);
    let xy: Mon<f64> = Mon::from(md2);
    let y2: Mon<f64> = Mon::from(md3);
    let yz: Mon<f64> = Mon::from(md4);
    let eight = Mon::one() * 8.;
    let z = Mon::zero();
    assert!(z < eight);
    assert!(xy < x2);
    let mut dp = vec![x2, xy, y2, yz, eight, z];
    dp.sort();
    println!("{:?}", dp);
}