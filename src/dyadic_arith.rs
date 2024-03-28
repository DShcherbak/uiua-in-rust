use crate::{UiuaElements, UiuaOperator, getter_macro};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DyadicArithmetic {
    Add,
    Sub,
    Mult,
    Div
}

getter_macro!{DyadicArithmetic}

getter_func!(add, Add);
getter_func!(sub, Sub);
getter_func!(mult, Mult);
getter_func!(div, Div);