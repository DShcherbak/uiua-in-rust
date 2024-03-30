use crate::{UiuaElements, UiuaOperator, getter_macro};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MonadicArray{
    Len,
    Iota
}

getter_macro!{MonadicArray}

getter_func!(lengt, Len);
getter_func!(iota, Iota);