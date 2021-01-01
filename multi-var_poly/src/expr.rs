use super::coef::*;
use super::mon::*;
use super::poly::*;
use super::ring::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pred {
    p: Poly,
    eq: bool, // true == '=', false == '≠'
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Ass {
        lv: Var,
        rv: Poly,
    },
    Skip,
    Seq {
        exprs: Vec<Expr>,
    },
    If {
        guard: Pred,
        the: Box<Expr>,
        els: Box<Expr>,
    },
    While {
        guard: Pred,
        c: Box<Expr>,
    },
}

#[test]
fn mannadiv_sample() {
    use std::collections::HashMap;
    // Init Ring
    // v -> x1, w -> x2, x -> y1, y -> y2, z -> y3
    let x1 = Var::new('v');
    let x2 = Var::new('w');
    let y1 = Var::new('x');
    let y2 = Var::new('y');
    let y3 = Var::new('z');

    let r = Ring::new(vec![x1, x2, y1, y2, y3]);
    let mut m = HashMap::new();
    m.insert(y1, 1);
    let pc11y1 = Poly::from((vec![Mon::from(m), Mon::one()], r.clone()));
    let pc12y2 = Poly::zero(r.clone());
    let mut m = HashMap::new();
    m.insert(y3, 1);
    let pc13y3 = Poly::from((vec![Mon::from(m), Mon::one() * -C::one()], r.clone()));
    // v -> x1, w -> x2, x -> y1, y -> y2, z -> y3
    let c1 = Expr::Seq {
        exprs: vec![
            Expr::Ass {
                lv: Var::new('x'),
                rv: pc11y1,
            },
            Expr::Ass {
                lv: Var::new('y'),
                rv: pc12y2,
            },
            Expr::Ass {
                lv: Var::new('z'),
                rv: pc13y3,
            },
        ],
    };

    // let c_while = If {
    //     guard:,
    //     the:,
    //     els:
    // }
    // guard polynomial
    // p = x2-y2-1
    let mut md1 = HashMap::new();
    md1.insert(x2, 1);
    let mut md2 = HashMap::new();
    md2.insert(y2, 1);
    let m1: Mon<C> = Mon::from(md1);
    let m2: Mon<C> = Mon::from(md2) * -C::one();
    let n_one: Mon<C> = Mon::one() * -C::one();
    let p = Poly::from((vec![m1, m2, n_one], r.clone()));
    // let w = Expr::While {
    //     guard: p,

    // }
}
