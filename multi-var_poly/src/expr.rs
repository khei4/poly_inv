use super::coef::*;
use super::constraints::*;
use super::expr_parse::*;
use super::mon::*;
use super::p_comb::*;
use super::poly::*;
use super::poly_parse::*;
use super::ring::*;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pred {
    pub p: Poly,
    pub eq: bool, // true == '=', false == '≠'
}

impl Pred {
    pub fn new(p: Poly, eq: bool) -> Self {
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
    let r = Ring::new();
    let x1 = r.borrow_mut().vextend(String::from("x1"));
    let x2 = r.borrow_mut().vextend(String::from("x2"));
    let y1 = r.borrow_mut().vextend(String::from("y1"));
    let y2 = r.borrow_mut().vextend(String::from("y2"));
    let y3 = r.borrow_mut().vextend(String::from("y3"));

    /*
        Initial Assignment
    */

    let p1y1 = Poly::zero(&r);
    let p2y2 = Poly::zero(&r);
    let p3y3 = Poly::from((vec![Mon::from((vec![(x1, 1)], &r))], &r));
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
    let pc11y1 = Poly::from((vec![Mon::from((vec![(y1, 1)], &r)), Mon::one(&r)], &r));
    let pc12y2 = Poly::zero(&r);
    let pc13y3 = Poly::from((
        vec![Mon::from((vec![(y3, 1)], &r)), Mon::one(&r) * -C::one()],
        &r,
    ));
    let c1 = Expr::Seq {
        exprs: vec![
            Expr::Ass { lv: y1, rv: pc11y1 },
            Expr::Ass { lv: y2, rv: pc12y2 },
            Expr::Ass { lv: y3, rv: pc13y3 },
        ],
    };
    // else clause
    let pc21y2 = Poly::from((vec![Mon::from((vec![(y2, 1)], &r)), Mon::one(&r)], &r));
    let pc22y3 = Poly::from((
        vec![Mon::from((vec![(y3, 1)], &r)), Mon::one(&r) * -C::one()],
        &r,
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
            Mon::from((vec![(x2, 1)], &r)),
            Mon::from((vec![(y2, 1)], &r)) * -C::one(),
            Mon::one(&r) * -C::one(),
        ],
        &r,
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
// 環に変数を追加しながら, 都合の良い形に変換する.
pub fn convert_from_parseresult(e: &E, r: &Rc<RefCell<Ring>>) -> Expr {
    match e {
        E::Ass { v, p } => {
            let v = r.borrow_mut().vextend(v.0.clone());
            let p = create_poly(p, r);
            Expr::Ass { lv: v, rv: p }
        }
        E::Skip => Expr::Skip,
        E::Seq { es } => {
            let mut exprs = vec![];
            for i in 0..es.len() {
                exprs.push(convert_from_parseresult(&es[i], r));
            }
            Expr::Seq { exprs }
        }
        E::If { guard, the, els } => {
            let e;
            match els {
                Some(els_exp) => e = convert_from_parseresult(els_exp, r),
                None => e = Expr::Skip,
            }
            Expr::If {
                guard: Pred::new(create_poly(&guard.p, r), guard.eq),
                the: Box::new(convert_from_parseresult(the, r)),
                els: Box::new(e),
            }
        }
        E::While { guard, body } => Expr::While {
            guard: Pred::new(create_poly(&guard.p, r), guard.eq),
            c: Box::new(convert_from_parseresult(body, r)),
        },
    }
}
