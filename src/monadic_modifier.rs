use crate::{UiuaElements, UiuaOperator, getter_macro};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MonadicModifier{
    Reduce
}

getter_macro!{MonadicModifier}

getter_func!(reduce, Reduce);