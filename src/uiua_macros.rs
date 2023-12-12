
use crate::stack::UiuaStack;
use crate::elems::UiuaElements;

macro_rules! build_one_ident {
    (+) => {
        UiuaElements::Plus
    };
    (-) => {
        UiuaElements::Minus
    };
    (.) => {
        UiuaElements::Dot
    };
    (,) => {
        UiuaElements::Comma
    };
    (/) => {
        UiuaElements::Div
    };
    (*) => {
        UiuaElements::Mult
    };
    ($a:ident) => {
        ($a).convert()
    };
    ($a:expr) => {
        UiuaElements::Elem($a)
    }
}

macro_rules! build_uiua_stack {
    // Base case: If there are no more tokens, stop the recursion.
    () => {{
        let u = UiuaStack { chars: vec![] };
        u
    }};

    // Match and process an identifier.
    ($id:ident $($rest:tt)*) => {{
    //  println!("Identifier: {}", stringify!($id));
        let c = build_one_ident!($id);
        let mut stack: UiuaStack = build_uiua_stack!($($rest)*);
        stack.chars.push(c);
        stack
    }};

    // Match and process a special symbol.
    ($sym:tt $($rest:tt)*) => {{
    // println!("Special Symbol: <{}>", stringify!($sym));
        let c = build_one_ident!($sym);
        let mut stack: UiuaStack = build_uiua_stack!($($rest)*);
        stack.chars.push(c);
        stack
    }};

    ($a:tt) => {{
        let c = build_one_ident!($a);
        let mut u = UiuaStack { chars: vec![] };
        u.chars.push(c);
        u
    }}
}


macro_rules! uiua {
    ($($x:tt)+) => {{
        let mut stack: UiuaStack = build_uiua_stack!($($x)+);
        let res = stack.calc();
        res
    }}
}

