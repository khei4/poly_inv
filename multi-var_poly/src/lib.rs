mod coef;
mod constraints;
mod expr;
mod mon;
mod poly;
mod ring;
mod temp;
#[allow(unused_imports)]
use coef::*;
#[allow(unused_imports)]
use constraints::*;
#[allow(unused_imports)]
use expr::*;
#[allow(unused_imports)]
use mon::*;
#[allow(unused_imports)]
use poly::*;
#[allow(unused_imports)]
use ring::*;
use temp::*;
#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
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

        println!("{:?}", gen_con(&c, PIdeal::most_gen(2, &r), Cs::new()));
    }

    #[test]
    fn assinment_test() {
        // 0 -> x1, 1 -> x2, 2 -> y1, 3 -> y2, 4 -> y3
        let x1 = Var::new(0);
        let x2 = Var::new(1);
        let y1 = Var::new(2);
        let y2 = Var::new(3);
        let y3 = Var::new(4);
        let r = Ring::new(vec![x1, x2, y1, y2, y3]);

        let pc13y3 = Poly::from((
            vec![Mon::from(vec![(y3, 1)]), Mon::one() * -C::one()],
            r.clone(),
        ));
        // y3 <- y3 - 1
        let e = Expr::Ass { lv: y3, rv: pc13y3 };
        let i = PIdeal::most_gen(1, &r);
        println!("init:{:?}", i);
        let (i, c) = gen_con(&e, i, Cs::new());
        println!("first:{:?}", i);

        let pc12y2 = Poly::zero(r.clone());
        let e = Expr::Ass { lv: y2, rv: pc12y2 };
        let (i, c) = gen_con(&e, i, c);
        println!("second:{:?}", i);

        let pc11y1 = Poly::from((vec![Mon::from(vec![(y1, 1)]), Mon::one()], r.clone()));
        let e = Expr::Ass { lv: y1, rv: pc11y1 };
        let (i, c) = gen_con(&e, i, c);
        println!("third:{:?}", i);
    }
    #[test]
    fn mannadiv() {
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
        // p = x2 - y2 - 1
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

        let w = Expr::While {
            guard: Pred::new(Poly::from((vec![Mon::from(y3)], r.clone())), false),
            c: Box::new(c_if),
        };
        let c = Expr::Seq {
            exprs: vec![c_init, w],
        };

        println!("{:?}", gen_con(&c, PIdeal::most_gen(2, &r), Cs::new()));
    }
}
