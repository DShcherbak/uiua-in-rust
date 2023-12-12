mod elems;
mod stack;
mod uiua_macros;

use elems::Convertable;
use elems::Explain;
use elems::UiuaElements;
use stack::UiuaStack;

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
    (:) => {
        UiuaElements::DoubleColon
    };
    (;) => {
        UiuaElements::Semicolon
    };
    ('∘') => {
        UiuaElements::Id
    };
    ($a:ident) => {
        ($a).convert()
    };
    ($a:expr) => {
        UiuaElements::Elem($a)
    };
}

macro_rules! build_uiua_stack {
    // Base case: If there are no more tokens, stop the recursion.
    () => {{
        let u = UiuaStack { chars: vec![] };
        u
    }};

    // Match and process an identifier.
    ($id:ident $($rest:tt)*) => {{
        let c = build_one_ident!($id);
        let mut stack: UiuaStack = build_uiua_stack!($($rest)*);
        stack.chars.push(c);
        stack
    }};

    // Match and process a special symbol.
    ($sym:tt $($rest:tt)*) => {{
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

fn main() {
    // Basic arithmetic operations
    let r1 = uiua!(+ 2 1);
    println!("+ 2 1 = {:?}", r1);

    let r2 = uiua!(+ 12345 54321);
    println!("+ 12345 54321 = {:?}", r2);

    // Accessing results as numbers
    let val = uiua!(+ 12345 54321).as_num().unwrap();
    println!("+ 12345 54321 = {:?}", val);

    // Combining variables and vectors
    let x = 1000;
    let v = vec![1, 2, 3];
    let r3 = uiua!(+ x v);
    println!("+ x v (x = 1000, v = [1, 2, 3]) = {:?}", r3);

    // Accessing results as vectors
    let r4 = uiua!(+ x v).as_vec().unwrap();
    println!("+ x v (x = 1000, v = [1, 2, 3]) = {:?}", r4);

    // Complex expressions
    let r5 = uiua!(+ - x 1 v).as_vec().unwrap();
    println!("+ - x 1 v (x = 1000, v = [1, 2, 3]) = {:?}", r5);

    let r6 = uiua!(+ - : x 1 v);
    println!("+ - : x 1 v (x = 1000, v = [1, 2, 3]) = {:?}", r6);

    // Spacing is not important as long as Rust can distinguish
    let r7 = uiua!(+-:x 1 v).as_vec().unwrap();
    println!("+-:x 1 v (x = 1000, v = [1, 2, 3]) = {:?}", r7);

    // Combining operations to create more complex functions
    // *. = ^2
    let r8 = uiua!(*. 25).as_num().unwrap();
    println!("*. 25 = {:?}", r8);

    // Handling error cases
    let r9 = uiua!(*25).as_err().unwrap();
    println!("* 25 = {:?}", r9);
}
