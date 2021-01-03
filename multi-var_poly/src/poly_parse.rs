use super::mon::*;
use super::p_comb::*;
use super::poly::*;
use super::ring::*;

// poly := term ('+' term | '-' term)*
// term := factor ('*' factor)*
// factor := value ('^' number)*
// value :=
// number

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
        Ok(("", expected_poly)),
        poly().parse("3 * x4 * y2 +  x1 ^ 3 * y1 ^ 2 * x1 ^ 4")
    );
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
fn primary<'a>() -> impl Parser<'a, P> {
    either(unsigned_number(), variable())
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
