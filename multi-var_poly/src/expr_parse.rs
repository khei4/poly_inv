use super::constraints::*;
use super::p_comb::*;
use super::poly::*;
use super::poly_parse::*;
use super::ring::*;
// 代入するだけで, 多項式に現れない変数はみない

// BNF
// program := expr*;
// expr := assign ';' | if_stmt | while_stmt | "skip" ';';
// assign := var '=' poly;
// if_stmt := "if" '(' pred ')' '{' stmt* '}' ("else" '{' stmt* '}')?;
// while_stmt := "while" '(' pred ')' '{' stmt* '}';
// pred := poly ('==' | '!=') poly;
// var := identifier
// やっぱり変数をRingに追加しながら構文解析みたいなのきついな...
// でも変数をStringにするのはやばそうだから, 一旦また別のEnumかませる
// というか, 評価する対象をこれにすればいいだけでは.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct V(pub String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pre {
    pub p: P,
    pub eq: bool, // true == '=', false == '≠'
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum E {
    Ass {
        v: V,
        p: P,
    },
    Skip,
    Seq {
        es: Vec<E>,
    },
    If {
        guard: Pre,
        the: Box<E>,
        els: Option<Box<E>>,
    },
    While {
        guard: Pre,
        body: Box<E>,
    },
}

fn var<'a>() -> impl Parser<'a, V> {
    identifier.map(|s| V(s))
}

#[test]
fn var_parser() {
    assert_eq!(Ok(("", V("x1".to_string()))), var().parse("x1"));
    assert_eq!(
        Ok(("^2 + y^2", V("x".to_string()))),
        var().parse("x^2 + y^2")
    );
    assert_eq!(Err("23x"), var().parse("23x"));
}

fn assign<'a>() -> impl Parser<'a, E> {
    pair(
        left(whitespace_wrap(var()), match_literal("=")),
        whitespace_wrap(poly()),
    )
    .map(|(v, p)| E::Ass { v, p })
}

#[test]
fn assign_parser() {
    let expected = E::Ass {
        v: V("x1".to_string()),
        p: P::Num(0),
    };
    assert_eq!(Ok(("", expected)), assign().parse("x1 = 0"));
    let expected = E::Ass {
        v: V("y4".to_string()),
        p: P::Num(7),
    };
    assert_eq!(Ok(("", expected)), assign().parse("y4 = 7"));
}

fn pred<'a>() -> impl Parser<'a, Pre> {
    pair(
        pair(
            poly(),
            whitespace_wrap(left(
                any_char.pred(|&c| c == '!' || c == '='),
                match_literal("="),
            )),
        ),
        poly(),
    )
    .map(|((p1, c), p2)| Pre {
        p: P::Sub {
            exp1: Box::new(p1),
            exp2: Box::new(p2),
        },
        eq: c == '=',
    })
}

#[test]
fn pred_parser() {
    let expected = Pre {
        p: P::Sub {
            exp1: Box::new(P::Num(0)),
            exp2: Box::new(P::Num(0)),
        },
        eq: true,
    };
    assert_eq!(Ok(("", expected)), pred().parse("0 == 0"));
    let expected = Pre {
        p: P::Sub {
            exp1: Box::new(P::Num(7)),
            exp2: Box::new(P::Num(0)),
        },
        eq: false,
    };
    assert_eq!(Ok(("", expected)), pred().parse("7 != 0"));
    let expected = Pre {
        p: P::Sub {
            exp1: Box::new(P::Num(0)),
            exp2: Box::new(P::Num(7)),
        },
        eq: false,
    };
    assert_eq!(Ok(("", expected)), pred().parse("0 != 7"));
}

fn if_cnd<'a>() -> impl Parser<'a, Pre> {
    right(
        pair(
            whitespace_wrap(match_literal("if")),
            whitespace_wrap(match_literal("(")),
        ),
        left(pred(), whitespace_wrap(match_literal(")"))),
    )
}

fn nested_program<'a>() -> impl Parser<'a, E> {
    right(
        whitespace_wrap(match_literal("{")),
        left(program(), whitespace_wrap(match_literal("}"))),
    )
}

fn if_stmt<'a>() -> impl Parser<'a, E> {
    if_cnd().and_then(|pred| {
        pair(
            nested_program(),
            one_or_zero(right(
                whitespace_wrap(match_literal("else")),
                nested_program(),
            )),
        )
        .map(move |(the, els)| match els {
            Some(e) => E::If {
                guard: pred.clone(),
                the: Box::new(the),
                els: Some(Box::new(e)),
            },
            None => E::If {
                guard: pred.clone(),
                the: Box::new(the),
                els: None,
            },
        })
    })
}

#[test]
fn if_parser() {
    let expected = E::If {
        guard: Pre {
            p: P::Sub {
                exp1: Box::new(P::Num(0)),
                exp2: Box::new(P::Num(0)),
            },
            eq: true,
        },
        the: Box::new(E::Ass {
            v: V("x1".to_string()),
            p: P::Num(0),
        }),
        els: None,
    };
    assert_eq!(
        Ok(("", expected)),
        if_stmt().parse("if (0 == 0) { x1 = 0; }")
    );
    let expected = E::If {
        guard: Pre {
            p: P::Sub {
                exp1: Box::new(P::Var("x".to_string())),
                exp2: Box::new(P::Num(0)),
            },
            eq: false,
        },
        the: Box::new(E::Seq {
            es: vec![
                E::Ass {
                    v: V("x1".to_string()),
                    p: P::Num(0),
                },
                E::Ass {
                    v: V("y".to_string()),
                    p: P::Num(1),
                },
            ],
        }),
        els: Some(Box::new(E::Ass {
            v: V("x1".to_string()),
            p: P::Var("y".to_string()),
        })),
    };
    assert_eq!(
        Ok(("", expected)),
        if_stmt().parse(
            r#"
        if (x != 0) 
            { x1 = 0; y = 1; } 
        else 
            {  x1 = y;      }
        "#
        )
    );
}

fn while_cnd<'a>() -> impl Parser<'a, Pre> {
    right(
        pair(
            whitespace_wrap(match_literal("while")),
            whitespace_wrap(match_literal("(")),
        ),
        left(pred(), whitespace_wrap(match_literal(")"))),
    )
}

fn while_stmt<'a>() -> impl Parser<'a, E> {
    while_cnd().and_then(|pred| {
        nested_program().map(move |c| E::While {
            guard: pred.clone(),
            body: Box::new(c),
        })
    })
}

#[test]
fn while_parser() {
    let expected = E::While {
        guard: Pre {
            p: P::Sub {
                exp1: Box::new(P::Num(0)),
                exp2: Box::new(P::Num(0)),
            },
            eq: true,
        },
        body: Box::new(E::Ass {
            v: V("x1".to_string()),
            p: P::Num(0),
        }),
    };
    assert_eq!(
        Ok(("", expected)),
        while_stmt().parse("while (0 == 0) { x1 = 0; }")
    );
}

fn skip<'a>() -> impl Parser<'a, E> {
    match_literal("skip").map(|()| E::Skip)
}
fn expr<'a>() -> impl Parser<'a, E> {
    either(
        left(assign(), match_literal(";")),
        either(
            if_stmt(),
            either(while_stmt(), left(skip(), match_literal(";"))),
        ),
    )
}

pub fn program<'a>() -> impl Parser<'a, E> {
    zero_or_more(whitespace_wrap(expr())).map(move |es| {
        if es.len() == 0 {
            E::Skip
        } else if es.len() == 1 {
            es[0].clone()
        } else {
            E::Seq { es }
        }
    })
}

#[test]
fn p_program_parser() {
    let c_then = E::Seq {
        es: vec![
            E::Ass {
                v: V("y1".to_string()),
                p: P::Add {
                    exp1: Box::new(P::Var("y1".to_string())),
                    exp2: Box::new(P::Num(1)),
                },
            },
            E::Ass {
                v: V("y2".to_string()),
                p: P::Num(0),
            },
            E::Ass {
                v: V("y3".to_string()),
                p: P::Sub {
                    exp1: Box::new(P::Var("y3".to_string())),
                    exp2: Box::new(P::Num(1)),
                },
            },
        ],
    };
    let c_else = E::Seq {
        es: vec![
            E::Ass {
                v: V("y2".to_string()),
                p: P::Add {
                    exp1: Box::new(P::Var("y2".to_string())),
                    exp2: Box::new(P::Num(1)),
                },
            },
            E::Ass {
                v: V("y3".to_string()),
                p: P::Sub {
                    exp1: Box::new(P::Var("y3".to_string())),
                    exp2: Box::new(P::Num(1)),
                },
            },
        ],
    };
    let c_while = E::While {
        guard: Pre {
            p: P::Sub {
                exp1: Box::new(P::Var("y3".to_string())),
                exp2: Box::new(P::Num(0)),
            },
            eq: false,
        },
        body: Box::new(E::If {
            guard: Pre {
                p: P::Sub {
                    exp1: Box::new(P::Add {
                        exp1: Box::new(P::Var("y2".to_string())),
                        exp2: Box::new(P::Num(1)),
                    }),
                    exp2: Box::new(P::Var("x2".to_string())),
                },
                eq: true,
            },
            the: Box::new(c_then),
            els: Some(Box::new(c_else)),
        }),
    };
    let expected = E::Seq {
        es: vec![
            E::Ass {
                v: V("y1".to_string()),
                p: P::Num(0),
            },
            E::Ass {
                v: V("y2".to_string()),
                p: P::Num(0),
            },
            E::Ass {
                v: V("y3".to_string()),
                p: P::Var("x1".to_string()),
            },
            c_while,
        ],
    };
    assert_eq!(
        Ok(("", expected)),
        program().parse(
            r#"
            y1 = 0;y2 = 0;y3 = x1;
            while(y3 != 0) {
                if (y2 + 1 == x2) {
                    y1 = y1 + 1;
                    y2 = 0;
                    y3 = y3 - 1;
                }
            
                else {
                    y2 = y2 + 1;
                    y3 = y3 - 1;
                }
            }"#
        )
    );
    // 変数一覧を取らなきゃいけなかった.
    let r = Ring::new();
    // gen_con_alt(&expected, PIdeal::most_gen(d: usize, r: &Rc<RefCell<Ring>>), mut c: Cs, r: &Rc<RefCell<Ring>>)
}
