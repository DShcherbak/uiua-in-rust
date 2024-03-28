#[macro_export]
macro_rules! build_one_ident {
    // Stack manipulation
    (.)   => { dupl() };
    (,)   => { over() };
    (:)   => { flip() };
    ('◌') => { pop() };
    ('∘') => { id() };

    // Monadic aryphmetic

    // Dyadic aryphmetic
    (+) => { add() };
    (-) => { sub() };
    (*) => { mult() };
    (/) => { div() };

    // elements
    ($a:ident) => {
        ($a).convert()
    };
    ($a:expr) => {
        $crate::elems::UiuaElements::Elem($a)
    };
}

#[macro_export]
macro_rules! build_uiua_stack {
    // Base case: If there are no more tokens, stop the recursion.
    () => {{
        let u = $crate::stack::UiuaStack { chars: vec![] };
        u
    }};

    // Match and process an identifier.
    ($id:ident $($rest:tt)*) => {{
        let c = build_one_ident!($id);
        let mut stack: $crate::stack::UiuaStack = build_uiua_stack!($($rest)*);
        stack.chars.push(c);
        stack
    }};

    // Match and process a special symbol.
    ($sym:tt $($rest:tt)*) => {{
        let c = build_one_ident!($sym);
        let mut stack: $crate::stack::UiuaStack = build_uiua_stack!($($rest)*);
        stack.chars.push(c);
        stack
    }};

    ($a:tt) => {{
        let c = build_one_ident!($a);
        let mut u = $crate::stack::UiuaStack { chars: vec![] };
        u.chars.push(c);
        u
    }}
}

#[macro_export]
macro_rules! uiua {
    ($($x:tt)+) => {{
        let mut stack: $crate::stack::UiuaStack = build_uiua_stack!($($x)+);
        let res = stack.calc();
        res
    }}
}