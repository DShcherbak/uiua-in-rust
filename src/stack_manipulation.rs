#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StackManipulation {
    Duplicate,
    Over,
    Flip,
    Pop,
    Id
}