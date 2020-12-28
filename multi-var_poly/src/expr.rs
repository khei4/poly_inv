#[derive(Debug, Clone)]
struct Pred {
    p: Poly,
    eq: bool, // true == '=', false == '≠'
}

enum Expr {
    Ass {
        lv: Var,
        rv: Poly,
    },
    Skip,
    Seq {
        first: Box<Expr>,
        second: Box<Expr>,
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
