#[derive(Debug, Clone)]
struct Pred {
    p: Polynomial<F>,
    eq: bool, // true == '=', false == '≠'
}

// unused, (only one variable is supported)
#[derive(Debug, Clone, Copy)]
struct Var {
    sym: char, // true == '=', false == '≠'
}

impl Var {
    fn new(c: char) -> Var {
        Var { sym: c }
    }
}

enum Expr {
    Ass {
        lv: Var,
        rv: Polynomial<F>,
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
