# Uiua-in-Rust Library

The uiua-in-rust library provides functionality to integrate the new Uiua programming language features seamlessly into Rust programs. The library offers specific macro that allows you to express computations in a concise and expressive manner, reminiscent of the Uiua language syntax.

## Installation

Clone this repo to your computer. In your Rust project, specify it as a local dependency: 

```toml
[dependencies]
uiua-in-rust = { path = "../uiua-in-rust" }
```


## Example usage

```rust
use uiua_in_rust::*;

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
    let r9 = uiua!(* 25).as_err().unwrap();
    println!("* 25 = {:?}", r9);
}
```

Result of this main function execution:
```
+ 2 1 = Elem(3)
+ 12345 54321 = Elem(66666)
+ 12345 54321 = 66666
+ x v (x = 1000, v = [1, 2, 3]) = Vector([1001, 1002, 1003])
+ x v (x = 1000, v = [1, 2, 3]) = [1001, 1002, 1003]
+ - x 1 v (x = 1000, v = [1, 2, 3]) = [1000, 1001, 1002]
+ - : x 1 v (x = 1000, v = [1, 2, 3]) = Vector([-998, -997, -996])
+-:x 1 v (x = 1000, v = [1, 2, 3]) = [-998, -997, -996]
*. 25 = 625
* 25 = "Not enough arguments for a function Mult"
```