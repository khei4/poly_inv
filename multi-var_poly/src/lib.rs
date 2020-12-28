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
pub struct Mon {
    // Var and Deg
    vars: HashMap<Var, usize>,
    coef: f64,
}

impl std::fmt::Debug for Mon {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // constant
        let mut res: String;
        // 定数がなぜか 表示されないけどいいや
        if self.vars.get(&Var::new('1')).is_some() || self.vars.get(&Var::new('0')).is_some() {
            res = format!("{}", self.coef);
        } else {
            // sign part
            if self.coef >= 0. && self.coef != 1. {
                res = String::from(format!("{}", self.coef));
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

// operation
impl Mon {
    // constantは, 変数
    fn one() -> Mon {
        Mon {
            vars: HashMap::new(),
            coef: 1.,
        }
    }

    // 番兵用/ zero
    fn zero() -> Mon {
        Mon {
            vars: HashMap::new(),
            coef: 0.,
        }
    }
}

impl From<HashMap<Var, usize>> for Mon {
    fn from(m: HashMap<Var, usize>) -> Self {
        Mon { vars: m, coef: 1. }
    }
}

impl std::ops::Mul<Mon> for Mon {
    type Output = Mon;
    fn mul(mut self, rhs: Mon) -> Self::Output {
        let mut n: Mon = Mon::one();
        // coeficient
        n.coef = self.coef * rhs.coef;
        // dicの統合
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
impl std::ops::Mul<f64> for Mon {
    type Output = Mon;
    fn mul(mut self, rhs: f64) -> Self::Output {
        self.coef *= rhs;
        if self.coef == 0. {
            self = Mon::zero();
        }
        self
    }
}

impl std::ops::MulAssign<Mon> for Mon {
    fn mul_assign(&mut self, rhs: Mon) {
        *self = self.clone() * rhs;
    }
}

impl Eq for Mon {}
impl std::cmp::PartialOrd for Mon {
    fn partial_cmp(&self, rhs: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(rhs))
    }
}
// Grevlex
impl std::cmp::Ord for Mon {
    fn cmp(&self, rhs: &Self) -> std::cmp::Ordering {
        if self.coef == 0. {
            return std::cmp::Ordering::Less;
        } else if rhs.coef == 0. {
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
            println!("{:?}", diff);
            for (v, d) in self.vars.iter() {
                diff.insert(*v, *d);
            }
            println!("{:?}", diff);
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

    let x2: Mon = Mon::from(md1);
    let xy: Mon = Mon::from(md2);
    let y2: Mon = Mon::from(md3);
    let yz: Mon = Mon::from(md4);
    let eight = Mon::one() * 8.;
    let z = Mon::zero();
    assert!(z < eight);
    assert!(xy < x2);
    let mut dp = vec![x2, xy, y2, yz, eight, z];
    dp.sort();
    println!("{:?}", dp);
}

/*
Polynomials
*/

use std::cmp::Reverse;
#[derive(PartialEq, Clone)]
pub struct Poly {
    mons: Vec<Reverse<Mon>>,
}

// display, debug
impl std::fmt::Debug for Poly {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut res = format!("{:?}", self.mons[0]);
        for i in 1..self.mons.len() {
            if self.mons[i].0.coef > 0. {
                res = format!("{}+{:?}", res, self.mons[i]);
            } else {
                res = format!("{}-{:?}", res, self.mons[i]);
            }
        }
        write!(f, "{}", res)
    }
}

// constructers

impl From<Vec<Mon>> for Poly {
    fn from(a: Vec<Mon>) -> Self {
        let mut mons = vec![];
        for m in a {
            mons.push(Reverse(m));
        }
        let mut p = Poly { mons };
        p.sort_sumup();
        p
    }
}

// methods

impl Poly {
    fn sort_sumup(&mut self) {
        // dummy monomial
        let dm = Reverse(Mon::zero());
        self.mons.push(dm.clone());
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
    // TOOO: multi-degree
    fn mdeg() {}
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

// TODO: Neg,

// TODO: Sub,

// TODO: Mul

// TODO: Par

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

// 足し算, スカラー倍だけ必要

// #[derive(Eq, PartialEq, PartialOrd, Ord, Clone, Copy, Hash)]
// enum LinExp {
//     Smul { par: Par, coef: f64 },
//     Add { l: LinExp, r: LinExp },
// }
// impl Par {
//     fn new(c: char) -> Par {
//         Par { sym: c }
//     }
// }

// impl std::fmt::Debug for Par {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         write!(f, "{}", self.sym)
//     }
// }

// TODO: Template

// TODO: // impl<T: ModI> std::ops::Mul<Poly> for Template {Template / Div

// impl<T: ModI> std::ops::Sub<Polynomial<T>> for Polynomial<T> {
//     type Output = Polynomial<T>;

//     fn sub(mut self, rhs: Polynomial<T>) -> Self::Output {
//         if self.coef.len() < rhs.coef.len() {
//             for i in 0..self.coef.len() {
//                 self.coef[i] -= rhs.coef[i];
//             }
//             self.shrink()
//         } else {
//             for i in 0..rhs.coef.len() {
//                 self.coef[i] -= rhs.coef[i];
//             }
//             self.shrink()
//         }
//     }
// }
// impl<T: ModI> std::ops::SubAssign<Polynomial<T>> for Polynomial<T> {
//     fn sub_assign(&mut self, rhs: Polynomial<T>) {
//         *self = self.clone() - rhs;
//     }
// }
// // convolution
// impl<T: ModI> std::ops::Mul<Polynomial<T>> for Polynomial<T> {
//     type Output = Polynomial<T>;

//     fn mul(mut self, mut rhs: Polynomial<T>) -> Self::Output {
//         Polynomial {
//             coef: single_convolution(&mut self.coef, &mut rhs.coef),
//         }
//         .shrink()
//     }
// }

// impl<T: ModI> std::ops::MulAssign<Polynomial<T>> for Polynomial<T> {
//     fn mul_assign(&mut self, rhs: Polynomial<T>) {
//         *self = self.clone() * rhs;
//     }
// }
// // scalar multiplication
// impl<T: ModI> std::ops::Mul<T> for Polynomial<T> {
//     type Output = Polynomial<T>;

//     fn mul(mut self, rhs: T) -> Self::Output {
//         for i in 0..self.coef.len() {
//             self.coef[i] *= rhs;
//         }
//         self
//     }
// }

// impl<T: ModI> std::ops::MulAssign<T> for Polynomial<T> {
//     fn mul_assign(&mut self, rhs: T) {
//         *self = self.clone() * rhs;
//     }
// }

// impl<T: ModI> std::ops::Div<Polynomial<T>> for Polynomial<T> {
//     type Output = Polynomial<T>;

//     fn div(mut self, rhs: Polynomial<T>) -> Self::Output {
//         if self.coef.len() < rhs.coef.len() {
//             Polynomial::new()
//         } else {
//             let n = self.coef.len();
//             let m = rhs.coef.len();
//             let res_size = n - m + 1;
//             let mut res = Polynomial::from(vec![T::default(); res_size]);
//             for i in 0..res_size {
//                 // if self.coef[n - (i + 1)] % rhs.coef[m - 1] != 0 {}
//                 let b = self.coef[n - (i + 1)] / rhs.coef[m - 1];
//                 res.coef[res_size - (i + 1)] = b;
//                 for j in 1..m {
//                     self.coef[n - (i + j)] -= b * rhs.coef[m - j];
//                 }
//             }
//             res
//         }
//     }
// }

// impl<T: ModI> std::ops::DivAssign<Polynomial<T>> for Polynomial<T> {
//     fn div_assign(&mut self, rhs: Polynomial<T>) {
//         *self = self.clone() / rhs;
//     }
// }

// impl<T: ModI> std::ops::Rem<Polynomial<T>> for Polynomial<T> {
//     type Output = Polynomial<T>;

//     fn rem(mut self, rhs: Polynomial<T>) -> Self::Output {
//         if self.coef.len() < rhs.coef.len() {
//             self
//         } else {
//             let n = self.coef.len();
//             let m = rhs.coef.len();
//             let res_size = n - m + 1;
//             let mut res = Polynomial::from(vec![T::default(); res_size]);
//             for i in 0..res_size {
//                 let b = self.coef[n - (i + 1)] / rhs.coef[m - 1];
//                 res.coef[res_size - (i + 1)] = b;
//                 for j in 1..m + 1 {
//                     self.coef[n - (i + j)] -= b * rhs.coef[m - j];
//                 }
//             }
//             self.shrink()
//         }
//     }
// }

// // methods
// impl<T: ModI> Polynomial<T> {
//     pub fn shrink(mut self) -> Self {
//         for i in (0..self.coef.len()).rev() {
//             if self.coef[i] != T::default() {
//                 self.coef.truncate(i + 1);
//                 break;
//             }
//         }
//         self
//     }

//     // many clone pow
//     pub fn pow(mut self, mut n: u64) -> Self {
//         let mut res: Polynomial<T> = Polynomial::from(vec![T::new(1)]);
//         while n > 0 {
//             if n & 1 == 1 {
//                 res *= self.clone();
//             }
//             self *= self.clone();
//             n >>= 1;
//         }
//         res
//     }

//     // degree
//     // いまのところ0は0を返しておく
//     pub fn deg(mut self) -> usize {
//         for i in (0..self.coef.len()).rev() {
//             if self.coef[i] != T::default() {
//                 self.coef.truncate(i + 1);
//                 return i;
//             }
//         }
//         return 0;
//     }
//     //
//     pub fn subs(self, v: i64) -> i64 {
//         let mut base = 1i64;
//         let mut res = 0;
//         for i in 0..self.coef.len() {
//             res += base * self.coef[i].val() as i64;
//             base *= v;
//         }
//         res
//     }
// }

fn main() {}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn mon_ord_test1() {
//         let x: Var = Var::new('x');
//         let y = Var::new('y');
//         let z = Var::new('z');

//         let mut md1 = HashMap::new();
//         md1.insert(x, 2);
//         let mut md2 = HashMap::new();
//         md2.insert(x, 1);
//         md2.insert(y, 1);
//         let mut md3 = HashMap::new();
//         md3.insert(y, 2);
//         let mut md4 = HashMap::new();
//         md4.insert(y, 1);
//         md4.insert(z, 1);

//         let x2: Mon = Mon::from(md1);
//         let xy: Mon = Mon::from(md2);
//         let y2: Mon = Mon::from(md3);
//         let yz: Mon = Mon::from(md4);
//         let eight = Mon::one() * 8.;
//         let z = Mon::zero();
//         let mut dp = vec![x2, xy, y2, yz, eight, z];
//         dp.sort();
//         println!("{:?}", dp);
//     }

//     #[test]
//     fn check_poly_addition() {
//         let x: Var = Var::new('x');
//         let y = Var::new('y');
//         let z = Var::new('z');

//         let mut md1 = HashMap::new();
//         md1.insert(x, 2);
//         let mut md2 = HashMap::new();
//         md2.insert(x, 1);
//         md2.insert(y, 1);
//         let mut md3 = HashMap::new();
//         md3.insert(y, 2);
//         let mut md4 = HashMap::new();
//         md4.insert(y, 1);
//         md4.insert(z, 1);

//         let yz: Mon = Mon::from(md4);
//         let x2: Mon = Mon::from(md1);
//         let xy: Mon = Mon::from(md2);
//         let y2: Mon = Mon::from(md3);
//         assert!(xy > yz);
//         let one: Mon = Mon::one();
//         let p1 = Poly::from(vec![x2, yz, one.clone() * 12.]);
//         let p2 = Poly::from(vec![xy, y2, one * 9.]);
//         let mut a = p1 + p2;
//         println!("{:?}", a);
//         a.mons.sort();
//         println!("{:?}", a);
//     }
// }
