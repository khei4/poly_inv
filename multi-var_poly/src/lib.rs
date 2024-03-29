mod coef;
mod constraints;
mod expr;
mod expr_parse;
mod mon;
mod p_comb;
mod poly;
mod poly_parse;
mod ring;
mod temp;
#[allow(unused_imports)]
use coef::*;
#[allow(unused_imports)]
use constraints::*;
#[allow(unused_imports)]
use expr::*;
use expr_parse::*;
#[allow(unused_imports)]
use mon::*;
use p_comb::*;
#[allow(unused_imports)]
use poly::*;
#[allow(unused_imports)]
use ring::*;
use temp::*;
#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    // #[test]
    // fn mannadiv_simple() {
    //     // Init Ring
    // let r = Ring::new();
    // let x1 = r.borrow_mut().vextend(String::from("x1"));
    // let x2 = r.borrow_mut().vextend(String::from("x2"));
    // let y1 = r.borrow_mut().vextend(String::from("y1"));
    // let y2 = r.borrow_mut().vextend(String::from("y2"));
    // let y3 = r.borrow_mut().vextend(String::from("y3"));
    //     /*
    //         Initial Assignment
    //     */
    //     let p1y1 = Poly::zero(&r);
    //     let p2y2 = Poly::zero(&r);
    //     let p3y3 = Poly::from((vec![Mon::from((vec![(x1, 1)], &r))], &r));
    //     let c_init = Expr::Seq {
    //         exprs: vec![
    //             Expr::Ass { lv: y1, rv: p1y1 },
    //             Expr::Ass { lv: y2, rv: p2y2 },
    //             Expr::Ass { lv: y3, rv: p3y3 },
    //         ],
    //     };

    //     /*
    //         Construct If
    //     */
    //     // then clause
    //     let pc11y1 = Poly::from((vec![Mon::from((vec![(y1, 1)], &r)), Mon::one(&r)], &r));
    //     let pc12y2 = Poly::zero(&r);
    //     let pc13y3 = Poly::from((
    //         vec![Mon::from((vec![(y3, 1)], &r)), Mon::one(&r) * -C::one()],
    //         &r,
    //     ));
    //     let c1 = Expr::Seq {
    //         exprs: vec![
    //             Expr::Ass { lv: y1, rv: pc11y1 },
    //             Expr::Ass { lv: y2, rv: pc12y2 },
    //             Expr::Ass { lv: y3, rv: pc13y3 },
    //         ],
    //     };
    //     // else clause
    //     let pc21y2 = Poly::from((vec![Mon::from((vec![(y2, 1)], &r)), Mon::one(&r)], &r));
    //     let pc22y3 = Poly::from((
    //         vec![Mon::from((vec![(y3, 1)], &r)), Mon::one(&r) * -C::one()],
    //         &r,
    //     ));
    //     let c2 = Expr::Seq {
    //         exprs: vec![
    //             Expr::Ass { lv: y2, rv: pc21y2 },
    //             Expr::Ass { lv: y3, rv: pc22y3 },
    //         ],
    //     };

    //     // guard polynomial
    //     // p = x2-y2-1
    //     let p = Poly::from((
    //         vec![
    //             Mon::from((vec![(x2, 1)], &r)),
    //             Mon::from((vec![(y2, 1)], &r)) * -C::one(),
    //             Mon::one(&r) * -C::one(),
    //         ],
    //         &r,
    //     ));
    //     let c_if = Expr::If {
    //         guard: Pred::new(p, true),
    //         the: Box::new(c1),
    //         els: Box::new(c2),
    //     };

    //     let c = Expr::Seq {
    //         exprs: vec![c_init, c_if],
    //     };
    //     let (i, c) = gen_con(&c, PIdeal::most_gen(2, &r), Cs::new());
    //     let c = c.add(Constraint(i, PIdeal::zero(&r)));
    //     println!("{:?}", c);
    // }

    // #[test]
    // fn assinment_test() {
    // let r = Ring::new();
    // let x1 = r.borrow_mut().vextend(String::from("x1"));
    // let x2 = r.borrow_mut().vextend(String::from("x2"));
    // let y1 = r.borrow_mut().vextend(String::from("y1"));
    // let y2 = r.borrow_mut().vextend(String::from("y2"));
    // let y3 = r.borrow_mut().vextend(String::from("y3"));

    //     let pc13y3 = Poly::from((
    //         vec![Mon::from((vec![(y3, 1)], &r)), Mon::one(&r) * -C::one()],
    //         &r,
    //     ));
    //     // y3 <- y3 - 1
    //     let e = Expr::Ass { lv: y3, rv: pc13y3 };
    //     let i = PIdeal::most_gen(1, &r);
    //     println!("init:{:?}", i);
    //     let (i, c) = gen_con(&e, i, Cs::new());
    //     println!("first:{:?}", i);

    //     let pc12y2 = Poly::zero(&r);
    //     let e = Expr::Ass { lv: y2, rv: pc12y2 };
    //     let (i, c) = gen_con(&e, i, c);
    //     println!("second:{:?}", i);

    //     let pc11y1 = Poly::from((vec![Mon::from((vec![(y1, 1)], &r)), Mon::one(&r)], &r));
    //     let e = Expr::Ass { lv: y1, rv: pc11y1 };
    //     let (i, c) = gen_con(&e, i, c);
    //     println!("third:{:?}", i);
    // }
}

// #[test]
// fn mannadiv() {
//     // Init Ring
//     let r = Ring::new();
//     let x1 = r.borrow_mut().vextend(String::from("x1"));
//     let x2 = r.borrow_mut().vextend(String::from("x2"));
//     let y1 = r.borrow_mut().vextend(String::from("y1"));
//     let y2 = r.borrow_mut().vextend(String::from("y2"));
//     let y3 = r.borrow_mut().vextend(String::from("y3"));

//     /*
//         Construct If
//     */
//     // then clause
//     let pc11y1 = Poly::from((vec![Mon::from((vec![(y1, 1)], &r)), Mon::one(&r)], &r));
//     let pc12y2 = Poly::zero(&r);
//     let pc13y3 = Poly::from((
//         vec![Mon::from((vec![(y3, 1)], &r)), Mon::one(&r) * -C::one()],
//         &r,
//     ));
//     let c1 = Expr::Seq {
//         exprs: vec![
//             Expr::Ass { lv: y1, rv: pc11y1 },
//             Expr::Ass { lv: y2, rv: pc12y2 },
//             Expr::Ass { lv: y3, rv: pc13y3 },
//         ],
//     };
//     // else clause
//     let pc21y2 = Poly::from((vec![Mon::from((vec![(y2, 1)], &r)), Mon::one(&r)], &r));
//     let pc22y3 = Poly::from((
//         vec![Mon::from((vec![(y3, 1)], &r)), Mon::one(&r) * -C::one()],
//         &r,
//     ));
//     let c2 = Expr::Seq {
//         exprs: vec![
//             Expr::Ass { lv: y2, rv: pc21y2 },
//             Expr::Ass { lv: y3, rv: pc22y3 },
//         ],
//     };

//     // guard polynomial
//     // p = x2 - y2 - 1
//     let p = Poly::from((
//         vec![
//             Mon::from((vec![(x2, 1)], &r)),
//             Mon::from((vec![(y2, 1)], &r)) * -C::one(),
//             Mon::one(&r) * -C::one(),
//         ],
//         &r,
//     ));
//     let c_if = Expr::If {
//         guard: Pred::new(p, true),
//         the: Box::new(c1),
//         els: Box::new(c2),
//     };

//     /*
//         Initial Assignment
//     */
//     let p1y1 = Poly::zero(&r);
//     let p2y2 = Poly::zero(&r);
//     let p3y3 = Poly::from((vec![Mon::from((vec![(x1, 1)], &r))], &r));
//     // Parserの都合で変える
//     // let mut c_init = Expr::Seq {
//     //     exprs: vec![
//     //         Expr::Ass { lv: y1, rv: p1y1 },
//     //         Expr::Ass { lv: y2, rv: p2y2 },
//     //         Expr::Ass { lv: y3, rv: p3y3 },
//     //     ],
//     // };
//     let w = Expr::While {
//         guard: Pred::new(Poly::from((vec![Mon::from((y3, &r))], &r)), false),
//         c: Box::new(c_if),
//     };
//     let c = Expr::Seq {
//         exprs: vec![
//             Expr::Ass { lv: y1, rv: p1y1 },
//             Expr::Ass { lv: y2, rv: p2y2 },
//             Expr::Ass { lv: y3, rv: p3y3 },
//             w,
//         ],
//     };
//     let g = Temp::most_gen(2, &r);
//     let (i, c) = gen_con_less_precise(&c, PIdeal::from(g.clone()), Cs::new());
//     // let (i, c) = gen_con(&c, PIdeal::from(g.clone()), Cs::new());
//     let c = c.add(Constraint(i, PIdeal::zero(&r)));
//     println!("{}", "target ideals");
//     for Constraint(i1, i2) in &c.items {
//         println!("i1={:?}", i1);
//         println!("i2={:?}", i2);
//     }
//     let le = LinearEquations::from((c, &r));
//     println!("{}", "===== solve these equations =====");
//     println!("{}", le);

//     match le.solve() {
//         Some(sol) => {
//             println!("{}", "===== solutions =====");
//             for s in &sol {
//                 println!("{:?} = {:?}", s.0, s.1);
//             }
//             println!(
//                 "{}",
//                 "===== substitute solutions to generic templates ====="
//             );
//             println!("{:?}", g.subs_pars(sol));
//         }
//         None => println!("Solution dosn't exist"),
//     }
// }

// #[test]
// fn mannadiv_parse() {
//     let r = Ring::new();
//     // 最初の代入は, 前のテストと整合性を取るためのもの
//     let c = convert_from_parseresult(
//         &program()
//             .parse(
//                 r#"
//                 x1 = x1; x2 = x2;
//                 y1 = 0;y2 = 0;y3 = x1;
//                 while ( y3 != 0 ) {
//                     if ( y2 + 1 == x2 ) {
//                         y1 = y1 + 1;
//                         y2 = 0;
//                         y3 = y3 - 1;
//                     }
//                     else {
//                         y2 = y2 + 1;
//                         y3 = y3 - 1;
//                     }
//                 }"#,
//             )
//             .map_or(E::Skip, |(_i, c)| c),
//         &r,
//     );
//     // println!("{:?}", c);
//     // println!("{:?}", r.borrow().vars);

//     let g = Temp::most_gen(2, &r);
//     // let (i, c) = gen_con(&c, PIdeal::from(g.clone()), Cs::new());
//     // let (i, c) = gen_con(&c, PIdeal::from(g.clone()), Cs::new());
//     let (i, c) = gen_con(&c, PIdeal::from(g.clone()), Cs::new());
//     let c = c.add(Constraint(i, PIdeal::zero(&r)));
//     println!("{}", "target ideals");
//     for Constraint(i1, i2) in &c.items {
//         println!("i1={:?}", i1);
//         println!("i2={:?}", i2);
//     }
//     let le = LinearEquations::from((c, &r));
//     println!("{}", "===== solve these equations =====");
//     println!("{}", le);

//     match le.solve() {
//         Some(sol) => {
//             println!("{}", "===== solutions =====");
//             for s in &sol {
//                 println!("{:?} = {:?}", s.0, s.1);
//             }
//             println!(
//                 "{}",
//                 "===== substitute solutions to generic templates ====="
//             );
//             println!("{:?}", g.subs_pars(sol));
//         }
//         None => println!("Solution dosn't exist"),
//     }
// }

#[test]
fn c_fall() {
    let r = Ring::new();
    let c = convert_from_parseresult(
        &program()
            .parse(
                r#"
                x = x0; v = v0; t = t0;
                while (t - a != 0) {
                    x = x + v * dt;
                    v = v - g * dt;
                    t = t + dt;
                }
                "#,
            )
            .map_or(E::Skip, |(_i, c)| c),
        &r,
    );
    // println!("{:?}", c);
    // println!("{:?}", r.borrow().vars);
    let g = Temp::most_gen(3, &r);
    // let (i, c) = gen_con(&c, PIdeal::from(g.clone()), Cs::new());
    let (i, c) = gen_con(&c, PIdeal::from(g.clone()), Cs::new());
    let c = c.add(Constraint(i, PIdeal::zero(&r)));
    println!("{}", "target ideals");
    for Constraint(i1, i2) in &c.items {
        println!("i1={:?}", i1);
        println!("i2={:?}", i2);
    }
    let le = LinearEquations::from((c, &r));
    println!("{}", "===== solve these equations =====");
    println!("{}", le);
    // parameter の集め方が間違っていたらしい
    // 乗算の自由度と, 最終的なInvariantの自由度は別??
    let inv;
    match le.solve() {
        Some(sol) => {
            le.check(&sol);
            println!("{}", "===== solutions =====");
            for s in &sol {
                println!("{:?} = {:?}", s.0, s.1);
            }
            println!(
                "{}",
                "===== substitute solutions to generic templates ====="
            );
            inv = g.subs_pars(sol);
            println!("{:?}", inv);
        }
        None => {
            println!("Solution dosn't exist");
            std::process::exit(0);
        }
    }
    // orthogonal components
    let pars = inv.get_pars();
    println!("{:?}", pars);
    for p in &pars {
        let mut e: Vec<(Par, LinExp)> = vec![];
        e.push((*p, LinExp::one()));
        for other_p in &pars {
            if p != other_p {
                e.push((*other_p, LinExp::zero()));
            }
        }
        println!("{:?}", inv.clone().subs_pars(e.clone()));
    }
}
