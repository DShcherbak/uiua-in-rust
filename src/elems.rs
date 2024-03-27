#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UiuaOperator {
    StackManipulation,
    MonadicAriphmethic,
    DyadicAriphmetic,
    MonadicArray,
    DyadicArray,
    MonadicModifier,
    DyadicModifier
    // Plus,
    // Minus,
    // Mult,
    // Div,
    // Dot,
    // Comma,
    // Semicolon,
    // DoubleColon,
    // Id,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UiuaElements {
    Operator(UiuaOperator),
    Elem(i32),
    Vector(Vec<i32>),
    Error(String),
}

pub trait Convertable {
    fn convert(&self) -> UiuaElements;
}

impl Convertable for i32 {
    fn convert(&self) -> UiuaElements {
        UiuaElements::Elem(*self)
    }
}

impl Convertable for Vec<i32> {
    fn convert(&self) -> UiuaElements {
        UiuaElements::Vector((*self).to_vec())
    }
}

pub trait Explain {
    fn as_num(&self) -> Option<i32>;
    fn as_vec(&self) -> Option<Vec<i32>>;
    fn as_err(&self) -> Option<String>;
}

impl Explain for UiuaElements {
    fn as_num(&self) -> Option<i32> {
        match &self {
            UiuaElements::Elem(x) => Some(*x),
            _ => None,
        }
    }

    fn as_err(&self) -> Option<String> {
        match &self {
            UiuaElements::Error(x) => Some(x.clone()),
            _ => None,
        }
    }

    fn as_vec(&self) -> Option<Vec<i32>> {
        match &self {
            UiuaElements::Vector(x) => Some(x.clone()),
            _ => None,
        }
    }
}
