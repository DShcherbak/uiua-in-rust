pub mod elems;
pub mod stack;
pub mod stack_calc;

pub mod perform;
pub use perform::Performer;

pub mod stack_manipulation;
pub mod dyadic_arith;

pub mod perform_stack_manipulation;
pub mod perform_dyadic_arith;

#[macro_use]
pub mod uiua_macros;

pub use elems::*;
pub use stack_manipulation::*;
pub use dyadic_arith::*;


#[cfg(test)]
mod tests {    

    // Import necessary items from the parent module
    use super::*;

    #[test]
    fn test_uiua_macro() {
        let result = uiua!('◌':.1);
        assert_eq!(result, UiuaElements::Elem(1));
    }

    #[test]
    fn test_uiua_add() {
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
    let r9 = uiua!(* 25).as_err().unwrap();
    println!("* 25 = {:?}", r9);

    let result = uiua!(*+1 2 3);
    assert_eq!(result, UiuaElements::Elem(9));

    let vv1 = vec![1, 20, 3, 4, 50];
    let vv2 = vec![10, 2, 30, 40, 5];
    let r10 = uiua!('↥' vv1 vv2).as_vec().unwrap();
    assert_eq!(r10, vec![10, 20, 30, 40, 50]);

}



    // Add more tests as needed...
}




// fn main() {

//     // Basic arithmetic operations
//     let r1 = uiua!(1);
//     println!("+ 2 1 = {:?}", r1);

//     // let r2 = uiua!(+ 12345 54321);
//     // println!("+ 12345 54321 = {:?}", r2);

//     // // Accessing results as numbers
//     // let val = uiua!(+ 12345 54321).as_num().unwrap();
//     // println!("+ 12345 54321 = {:?}", val);

//     // // Combining variables and vectors
//     // let x = 1000;
//     // let v = vec![1, 2, 3];
//     // let r3 = uiua!(+ x v);
//     // println!("+ x v (x = 1000, v = [1, 2, 3]) = {:?}", r3);

//     // // Accessing results as vectors
//     // let r4 = uiua!(+ x v).as_vec().unwrap();
//     // println!("+ x v (x = 1000, v = [1, 2, 3]) = {:?}", r4);

//     // // Complex expressions
//     // let r5 = uiua!(+ - x 1 v).as_vec().unwrap();
//     // println!("+ - x 1 v (x = 1000, v = [1, 2, 3]) = {:?}", r5);

//     // let r6 = uiua!(+ - : x 1 v);
//     // println!("+ - : x 1 v (x = 1000, v = [1, 2, 3]) = {:?}", r6);

//     // // Spacing is not important as long as Rust can distinguish
//     // let r7 = uiua!(+-:x 1 v).as_vec().unwrap();
//     // println!("+-:x 1 v (x = 1000, v = [1, 2, 3]) = {:?}", r7);

//     // // Combining operations to create more complex functions
//     // // *. = ^2
//     // let r8 = uiua!(*. 25).as_num().unwrap();
//     // println!("*. 25 = {:?}", r8);

//     // let r81 = uiua!('∘' 25).as_num().unwrap();
//     // println!("'∘'. 25 = {:?}", r81);

//     // // Handling error cases
//     // let r9 = uiua!(*25).as_err().unwrap();
//     // println!("* 25 = {:?}", r9);
// }
