use super::coef::*;
use super::mon::*;
use super::poly::*;
use super::ring::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pred {
    p: Poly,
    eq: bool, // true == '=', false == '≠'
}

impl Pred {
    fn new(p: Poly, eq: bool) -> Self {
        Pred { p, eq }
    }
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
fn mannadiv_simple() {
    // Init Ring
    // 0 -> x1, 1 -> x2, 2 -> y1, 3 -> y2, 4 -> y3
    let x1 = Var::new(0);
    let x2 = Var::new(1);
    let y1 = Var::new(2);
    let y2 = Var::new(3);
    let y3 = Var::new(4);
    let r = Ring::new(vec![x1, x2, y1, y2, y3]);

    /*
        Initial Assignment
    */

    let p1y1 = Poly::zero(r.clone());
    let p2y2 = Poly::zero(r.clone());
    let p3y3 = Poly::from((vec![Mon::from(vec![(x1, 1)])], r.clone()));
    let c_init = Expr::Seq {
        exprs: vec![
            Expr::Ass { lv: y1, rv: p1y1 },
            Expr::Ass { lv: y2, rv: p2y2 },
            Expr::Ass { lv: y3, rv: p3y3 },
        ],
    };

    /*
        Construct If
    */
    // then clause
    let pc11y1 = Poly::from((vec![Mon::from(vec![(y1, 1)]), Mon::one()], r.clone()));
    let pc12y2 = Poly::zero(r.clone());
    let pc13y3 = Poly::from((
        vec![Mon::from(vec![(y3, 1)]), Mon::one() * -C::one()],
        r.clone(),
    ));
    let c1 = Expr::Seq {
        exprs: vec![
            Expr::Ass { lv: y1, rv: pc11y1 },
            Expr::Ass { lv: y2, rv: pc12y2 },
            Expr::Ass { lv: y3, rv: pc13y3 },
        ],
    };
    // else clause
    let pc21y2 = Poly::from((vec![Mon::from(vec![(y2, 1)]), Mon::one()], r.clone()));
    let pc22y3 = Poly::from((
        vec![Mon::from(vec![(y3, 1)]), Mon::one() * -C::one()],
        r.clone(),
    ));
    let c2 = Expr::Seq {
        exprs: vec![
            Expr::Ass { lv: y2, rv: pc21y2 },
            Expr::Ass { lv: y3, rv: pc22y3 },
        ],
    };

    // guard polynomial
    // p = x2-y2-1
    let p = Poly::from((
        vec![
            Mon::from(vec![(x2, 1)]),
            Mon::from(vec![(y2, 1)]) * -C::one(),
            Mon::one() * -C::one(),
        ],
        r.clone(),
    ));
    let c_if = Expr::If {
        guard: Pred::new(p, true),
        the: Box::new(c1),
        els: Box::new(c2),
    };

    let c = Expr::Seq {
        exprs: vec![c_init, c_if],
    };
}
