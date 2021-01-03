use super::coef::*;
use super::mon::*;
use super::p_comb::*;
use super::poly::*;
use super::ring::*;
use std::cell::RefCell;
use std::rc::Rc;
// poly := term ('+' term | '-' term)*;
// term := factor ('*' factor)*;
// factor := unary ('^' number)*;
// unary := ('+'|'-')* primary;
// primary := number | var | '(' poly ')';
// number := digit+;
// var := letter ( letter | digit )*;
// letter = ("a" | "b" | ... | "z" | "A" | ... | "Z");
// digit = "0" | "1" | "2" | ... | "9";

// AST of poly
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum P {
    Add { exp1: Box<P>, exp2: Box<P> },
    Sub { exp1: Box<P>, exp2: Box<P> },
    Mul { exp1: Box<P>, exp2: Box<P> },
    Pow { exp1: Box<P>, exp2: Box<P> },
    Neg(Box<P>),
    Var(String),
    Num(i64),
}

fn unsigned_number<'a>() -> impl Parser<'a, P> {
    one_or_more(any_char.pred(|c| c.is_numeric())).map(|chars| {
        P::Num(
            chars
                .into_iter()
                .fold(0, |s, c| s * 10 + c.to_digit(10).expect("") as i64),
        )
    })
}
#[test]
fn number_parser() {
    assert_eq!(Ok(("", P::Num(64))), unsigned_number().parse("64"));
    assert_eq!(Ok(("", P::Num(12333))), unsigned_number().parse("12333"));
    assert_eq!(Ok(("", P::Num(0))), unsigned_number().parse("0"));
    assert_eq!(Err(""), unsigned_number().parse(""));
    assert_eq!(Err("-123"), unsigned_number().parse("-123"));
}

fn variable<'a>() -> impl Parser<'a, P> {
    identifier.map(|s| P::Var(s))
}

#[test]
fn variable_parser() {
    assert_eq!(Ok(("", P::Var("x1".to_string()))), variable().parse("x1"));
    assert_eq!(
        Ok(("^2 + y^2", P::Var("x".to_string()))),
        variable().parse("x^2 + y^2")
    );
    assert_eq!(Err("23x"), variable().parse("23x"));
}

fn primary<'a>() -> impl Parser<'a, P> {
    either(
        unsigned_number(),
        either(
            variable(),
            right(
                match_literal("("),
                left(whitespace_wrap(poly()), match_literal(")")),
            ),
        ),
    )
}

fn unary<'a>() -> impl Parser<'a, P> {
    zero_or_more(whitespace_wrap(any_char.pred(|c| *c == '+' || *c == '-'))).and_then(|vc| {
        primary().map(move |p| {
            if vc.iter().filter(|&c| *c == '-').count() % 2 != 0 {
                P::Neg(Box::new(p))
            } else {
                p
            }
        })
    })
}

fn factor<'a>() -> impl Parser<'a, P> {
    unary().and_then(|val| {
        zero_or_more(right(
            whitespace_wrap(match_literal("^")),
            unsigned_number(),
        ))
        .map(move |mut ps| {
            if ps.len() == 0 {
                val.clone()
            } else {
                let mut pow: P = ps.pop().unwrap();
                let mut res = val.clone();
                res = P::Pow {
                    exp1: Box::new(res),
                    exp2: Box::new(pow),
                };
                while let Some(p) = ps.pop() {
                    match &mut res {
                        P::Pow { exp2, .. } => {
                            pow = P::Pow {
                                exp1: Box::new(p),
                                exp2: exp2.clone(),
                            };
                            *exp2 = Box::new(pow);
                        }
                        _ => unreachable!(),
                    }
                }
                res
            }
        })
    })
}
#[test]
fn factor_parser() {
    let expected_factor1 = P::Pow {
        exp1: Box::new(P::Var("x1".to_string())),
        exp2: Box::new(P::Pow {
            exp1: Box::new(P::Num(3)),
            exp2: Box::new(P::Num(2)),
        }),
    };

    assert_eq!(Ok(("", expected_factor1)), factor().parse("x1 ^ 3 ^ 2"));

    let expected_factor2 = P::Pow {
        exp1: Box::new(P::Var("x1".to_string())),
        exp2: Box::new(P::Pow {
            exp1: Box::new(P::Num(3)),
            exp2: Box::new(P::Pow {
                exp1: Box::new(P::Num(3)),
                exp2: Box::new(P::Num(2)),
            }),
        }),
    };
    assert_eq!(Ok(("", expected_factor2)), factor().parse("x1 ^ 3 ^ 3 ^ 2"));
}

fn term<'a>() -> impl Parser<'a, P> {
    factor().and_then(|val| {
        zero_or_more(right(whitespace_wrap(match_literal("*")), factor())).map(
            move |mut factors| {
                if factors.len() == 0 {
                    // closureのmove, borrowingまったくわかってない...
                    val.clone()
                } else {
                    let mut res = val.clone();
                    factors.reverse();
                    while let Some(f) = factors.pop() {
                        res = P::Mul {
                            exp1: Box::new(res),
                            exp2: Box::new(f),
                        };
                    }
                    res
                }
            },
        )
    })
}

#[test]
fn term_parser() {
    let expected_term = P::Mul {
        exp1: Box::new(P::Mul {
            exp1: Box::new(P::Pow {
                exp1: Box::new(P::Var("x1".to_string())),
                exp2: Box::new(P::Num(3)),
            }),
            exp2: Box::new(P::Pow {
                exp1: Box::new(P::Var("y1".to_string())),
                exp2: Box::new(P::Num(2)),
            }),
        }),
        exp2: Box::new(P::Pow {
            exp1: Box::new(P::Var("x1".to_string())),
            exp2: Box::new(P::Num(4)),
        }),
    };

    assert_eq!(
        Ok(("", expected_term)),
        term().parse("x1 ^ 3 * y1 ^ 2 * x1 ^ 4")
    );
}

fn poly<'a>() -> impl Parser<'a, P> {
    term().and_then(|val| {
        zero_or_more(pair(
            whitespace_wrap(any_char.pred(|c| *c == '+' || *c == '-')),
            term(),
        ))
        .map(move |mut terms| {
            if terms.len() == 0 {
                val.clone()
            } else {
                let mut res = val.clone();
                terms.reverse();
                while let Some((op, t)) = terms.pop() {
                    match op {
                        '+' => {
                            res = P::Add {
                                exp1: Box::new(res),
                                exp2: Box::new(t),
                            }
                        }
                        '-' => {
                            res = P::Sub {
                                exp1: Box::new(res),
                                exp2: Box::new(t),
                            }
                        }
                        _ => unreachable!(),
                    }
                }
                res
            }
        })
    })
}

#[test]
fn poly_parser() {
    let expected_term_left = P::Mul {
        exp1: Box::new(P::Mul {
            exp1: Box::new(P::Num(3)),
            exp2: Box::new(P::Var("x4".to_string())),
        }),
        exp2: Box::new(P::Var("y2".to_string())),
    };
    let expected_term_right = P::Mul {
        exp1: Box::new(P::Mul {
            exp1: Box::new(P::Pow {
                exp1: Box::new(P::Var("x1".to_string())),
                exp2: Box::new(P::Num(3)),
            }),
            exp2: Box::new(P::Pow {
                exp1: Box::new(P::Var("y1".to_string())),
                exp2: Box::new(P::Num(2)),
            }),
        }),
        exp2: Box::new(P::Pow {
            exp1: Box::new(P::Var("x1".to_string())),
            exp2: Box::new(P::Num(4)),
        }),
    };
    let expected_poly = P::Add {
        exp1: Box::new(expected_term_left),
        exp2: Box::new(expected_term_right),
    };

    assert_eq!(
        Ok(("", expected_poly.clone())),
        poly().parse("3 * x4 * y2 +  x1 ^ 3 * y1 ^ 2 * x1 ^ 4")
    );
    let powed_poly = P::Pow {
        exp1: Box::new(expected_poly),
        exp2: Box::new(P::Num(3)),
    };
    assert_eq!(
        Ok(("", powed_poly)),
        poly().parse("(       3 * x4 * y2 +  x1 ^ 3 * y1 ^ 2 * x1 ^ 4 ) ^ 3")
    );
}

// powをくりかえし許すからこんなことに...
fn pow_calc(p: &P) -> usize {
    match p {
        P::Num(n) if *n >= 0 => *n as usize,
        P::Pow { exp1, exp2 } => pow_calc(exp1).pow(pow_calc(exp2) as u32),
        _ => unreachable!(),
    }
}

fn create_poly(p: &P, r: &Rc<RefCell<Ring>>) -> Poly {
    match p {
        P::Add { exp1, exp2 } => create_poly(exp1, r) + create_poly(exp2, r),
        P::Mul { exp1, exp2 } => create_poly(exp1, r) * create_poly(exp2, r),
        P::Sub { exp1, exp2 } => create_poly(exp1, r) - create_poly(exp2, r),
        // 先にusizeのpowを計算してしまう
        P::Pow { exp1, exp2 } => create_poly(exp1, r).pow(pow_calc(exp2)),
        P::Neg(exp) => -create_poly(exp, r),
        P::Num(n) => Poly::from((C::new(*n, 1), r)),
        P::Var(s) => {
            let v = r.borrow_mut().vextend(s.clone());
            Poly::from((v, r))
        }
    }
}

#[test]
fn poly_construct() {
    let expected_term_left = P::Mul {
        exp1: Box::new(P::Mul {
            exp1: Box::new(P::Num(3)),
            exp2: Box::new(P::Var("x4".to_string())),
        }),
        exp2: Box::new(P::Var("y2".to_string())),
    };
    let expected_term_right = P::Mul {
        exp1: Box::new(P::Mul {
            exp1: Box::new(P::Pow {
                exp1: Box::new(P::Var("x1".to_string())),
                exp2: Box::new(P::Num(3)),
            }),
            exp2: Box::new(P::Pow {
                exp1: Box::new(P::Var("y1".to_string())),
                exp2: Box::new(P::Num(2)),
            }),
        }),
        exp2: Box::new(P::Pow {
            exp1: Box::new(P::Var("x1".to_string())),
            exp2: Box::new(P::Num(4)),
        }),
    };
    let expected_poly = P::Add {
        exp1: Box::new(expected_term_left),
        exp2: Box::new(expected_term_right),
    };

    assert_eq!(
        Ok(("", expected_poly.clone())),
        poly().parse("3 * x4 * y2 +  x1 ^ 3 * y1 ^ 2 * x1 ^ 4")
    );
    let powed_poly = P::Pow {
        exp1: Box::new(expected_poly),
        exp2: Box::new(P::Num(3)),
    };
    assert_eq!(
        Ok(("", powed_poly)),
        poly().parse("(       3 * x4 * y2 +  x1 ^ 3 * y1 ^ 2 * x1 ^ 4 ) ^ 3")
    );
    let r = Ring::new(vec![]);
    match poly().parse("(       3 * x4 * y2 +  x1 ^ 3 * y1 ^ 2 * x1 ^ 4 ) ") {
        Ok((_s, p)) => println!("{:?}", create_poly(&p, &r)),
        Err(_) => unreachable!(),
    }
}
