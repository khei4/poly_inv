use super::p_comb::*;
use super::poly::*;
use super::poly_parse::*;
use super::ring::*;
// 代入するだけで, 多項式に現れない変数はみない

// BNF
// program := stmt*;
// stmd := expr ';';
// expr := assign | if_stmt | while_stmt | "skip";
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
        exprs: Vec<E>,
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

// fn if_stmt<'a>() -> impl Parser<'a, E> {
//     right(
//         pair(
//             whitespace_wrap(match_literal("if")),
//             whitespace_wrap(match_literal("(")),
//         ),
//         left(pred(), match_literal(")")),
//     )
//     .and_then(|pred| {
//         right(
//             whitespace_wrap(match_literal("{")),
//             left(program(), whitespace_wrap(match_literal("}"))),
//         )
//         .and_then(|the| {
//             one_or_zero(
//                 right(whitespace_wrap(match_literal("else")),
//                     right(
//                         whitespace_wrap(match_literal("{")),
//                         left(program(), whitespace_wrap(match_literal("}"))),
//                     ),
//             ))
//             .map(move |els| E::If { guard: pred, the, els })
//     })
// }

// fn term<'a>() -> impl Parser<'a, P> {
//     factor().and_then(|val| {
//         zero_or_more(right(whitespace_wrap(match_literal("*")), factor())).map(
//             move |mut factors| {
//                 if factors.len() == 0 {
//                     // closureのmove, borrowingまったくわかってない...
//                     val.clone()
//                 } else {
//                     let mut res = val.clone();
//                     factors.reverse();
//                     while let Some(f) = factors.pop() {
//                         res = P::Mul {
//                             exp1: Box::new(res),
//                             exp2: Box::new(f),
//                         };
//                     }
//                     res
//                 }
//             },
//         )
//     })
// }
