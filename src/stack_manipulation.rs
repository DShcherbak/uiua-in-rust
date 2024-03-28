use crate::{UiuaElements, UiuaOperator};

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StackManipulation {
    Duplicate,
    Over,
    Flip,
    Pop,
    Id
}

pub fn dupl() -> UiuaElements {
    UiuaElements::Operator(UiuaOperator::StackManipulation(StackManipulation::Duplicate))
}

pub fn over() -> UiuaElements {
    UiuaElements::Operator(UiuaOperator::StackManipulation(StackManipulation::Over))
}

pub fn flip() -> UiuaElements {
    UiuaElements::Operator(UiuaOperator::StackManipulation(StackManipulation::Flip))
}

pub fn pop() -> UiuaElements {
    UiuaElements::Operator(UiuaOperator::StackManipulation(StackManipulation::Pop))
}

pub fn id() -> UiuaElements {
    UiuaElements::Operator(UiuaOperator::StackManipulation(StackManipulation::Id))
}
