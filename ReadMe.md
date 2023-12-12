# Uiua-in-Rust Library

The uiua-in-rust library provides functionality to integrate the new Uiua programming language features seamlessly into Rust programs. The library offers specific macro that allows you to express computations in a concise and expressive manner, reminiscent of the Uiua language syntax.

## Example usage

```rust
use uiua_in_rust::uiua;

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

    // Using aliases for operators
    let r6 = uiua!(+ - : x 1 v);
    println!("+ - : x 1 v (x = 1000, v = [1, 2, 3]) = {:?}", r6);

    // Alternative syntax for operators
    let r7 = uiua!(+-:x 1 v).as_vec().unwrap();
    println!("+-:x 1 v (x = 1000, v = [1, 2, 3]) = {:?}", r7);

    // Using a dot for multiplication
    let r8 = uiua!(* . 25).as_num().unwrap();
    println!("* . 25 = {:?}", r8);

    // Handling error cases
    let r9 = uiua!(* 25).as_err().unwrap();
    println!("* 25 = {:?}", r9);
}
```