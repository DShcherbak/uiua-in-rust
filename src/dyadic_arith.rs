use crate::{UiuaElements, UiuaOperator, getter_macro};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DyadicArithmetic {
    Add,
    Sub,
    Mult,
    Div,
    Eq,
    Neq,
    LT,
    GT,
    LET,
    GET,
    Mod,
    Exp,
  //  Log,
    Min,
    Max,
  //  Atan
}

getter_macro!{DyadicArithmetic}

getter_func!(add, Add);
getter_func!(sub, Sub);
getter_func!(mult, Mult);
getter_func!(div, Div);
getter_func!(eq, Eq);
getter_func!(neq, Neq);
getter_func!(lt, LT);
getter_func!(gt, GT);
getter_func!(loet, LET);
getter_func!(goet, GET);
getter_func!(modulo, Mod);
getter_func!(exp, Exp);
//getter_func!(log_n, Log);
getter_func!(min, Min);
getter_func!(max, Max);
//getter_func!(atan, Atan);