use super::coef::*;
use super::mon::*;
use std::cmp::Reverse;
#[derive(PartialEq, Clone)]
pub struct Temp {
    mons: Vec<Reverse<Mon<LinExp>>>,
}

impl std::fmt::Debug for Temp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // TODO: Change
        let mut res = format!("{:?}", self.mons[0]);
        for i in 1..self.mons.len() {
            if self.mons[i].0.coef >= LinExp::zero() {
                res = format!("{}+{:?}", res, self.mons[i]);
            } else {
                res = format!("{}-{:?}", res, self.mons[i]);
            }
        }
        write!(f, "{}", res)
    }
}

// constructers
impl From<Vec<Mon<LinExp>>> for Temp {
    fn from(a: Vec<Mon<LinExp>>) -> Self {
        let mut mons = vec![];
        for m in a {
            mons.push(Reverse(m));
        }
        let mut p = Temp { mons };
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
    // TOOO: multi-degree
    fn mdeg() {}
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

// #[test]
// fn check_Temp_addition() {
//     use std::collections::HashMap;
//     let x: Var = Var::new('x');
//     let y = Var::new('y');
//     let z = Var::new('z');

//     let mut md1 = HashMap::new();
//     md1.insert(x, 2);
//     let mut md2 = HashMap::new();
//     md2.insert(x, 1);
//     md2.insert(y, 1);
//     let mut md3 = HashMap::new();
//     md3.insert(y, 2);
//     let mut md4 = HashMap::new();
//     md4.insert(y, 1);
//     md4.insert(z, 1);

//     let yz: Mon<f64> = Mon::from(md4);
//     let x2: Mon<f64> = Mon::from(md1);
//     let xy: Mon<f64> = Mon::from(md2);
//     let y2: Mon<f64> = Mon::from(md3);
//     assert!(xy > yz);
//     let one: Mon<f64> = Mon::one();
//     let p1 = Temp::from(vec![x2, yz, one.clone() * 12.]);
//     let p2 = Temp::from(vec![xy, y2, one * 9.]);
//     let mut a = p1 + p2;
//     println!("{:?}", a);
//     a.sort_sumup();
//     println!("{:?}", a);
// }