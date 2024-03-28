use crate::{UiuaElements, UiuaOperator, getter_macro};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StackManipulation {
    Duplicate,
    Over,
    Flip,
    Pop,
    Id
}

getter_macro!{StackManipulation}

getter_func!(dupl, Duplicate);
getter_func!(over, Over);
getter_func!(flip, Flip);
getter_func!(pop, Pop);
getter_func!(id, Id);