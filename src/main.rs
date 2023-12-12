
mod elems;
mod stack;
mod uiua_macros;

use stack::UiuaStack;
use elems::UiuaElements;
use elems::Convertable;
use elems::Explain;

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
        UiuaElements::Semicolon
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


fn main() {
    let r1 = uiua!(+ 2 1);
    println!("+ 2 1 = {:?}", r1);
    
    let r2 = uiua!(+ 12345 54321);
    println!("+ 12345 54321 = {:?}", r2);
    let val = uiua!(+ 12345 54321).as_num().unwrap();
    println!("+ 12345 54321 = {:?}", val);

    let x  = 1000;
    let v = vec![1,2,3];
    let r2 = uiua!(+ x v);
    println!("+ x v (x = 1000, v = [1,2,3]) = {:?}", r2);
    let r2 = uiua!(+ x v).as_vec().unwrap();
    println!("+ x v (x = 1000, v = [1,2,3]) = {:?}", r2);

    let x = 1000;
    let v = vec![1,2,3];
    let r2 = uiua!(+ - x 1 v).as_vec().unwrap();
    println!("+ - x 1 v (x = 1000, v = [1,2,3]) = {:?}", r2);

    let x = 1000;
    let v = vec![1,2,3];
    let r2 = uiua!(+ - : x 1 v);
    println!("+ - : x 1 v (x = 1000, v = [1,2,3]) = {:?}", r2);
    let r2 = uiua!(+ - : x 1 v);
    println!("+ - : x 1 v (x = 1000, v = [1,2,3]) = {:?}", r2);

    let x = 1000;
    let v = vec![1,2,3];
    let r2 = uiua!(+-:x 1 v).as_vec().unwrap();
    println!("+-:x 1 v (x = 1000, v = [1,2,3]) = {:?}", r2);

    let r2 = uiua!(* . 25).as_num().unwrap();
    println!("* . 25 = {:?}", r2);

    let r2 = uiua!(* 25).as_err().unwrap();
    println!("* 25 = {:?}", r2);
}