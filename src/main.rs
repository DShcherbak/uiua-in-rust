mod elems;
mod stack;
#[macro_use]
mod uiua_macros;

use elems::Convertable;
use elems::Explain;
use elems::UiuaElements;
use stack::UiuaStack;


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

    let r81 = uiua!('∘' 25).as_num().unwrap();
    println!("'∘'. 25 = {:?}", r81);

    // Handling error cases
    let r9 = uiua!(*25).as_err().unwrap();
    println!("* 25 = {:?}", r9);
}
