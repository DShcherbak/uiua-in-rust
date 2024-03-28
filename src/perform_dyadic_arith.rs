use crate::{dyadic_arith::*, Performer};
use crate::elems::UiuaElements;

// pub fn get_binary(f: UiuaElements) -> (Box<dyn Fn(i32, i32) -> i32>, String) {
//     match f {
//         // UiuaElements::Plus => (Box::new(|x, y| x + y), "+".to_string()),
//         // UiuaElements::Minus => (Box::new(|x, y| x - y), "-".to_string()),
//         // UiuaElements::Mult => (Box::new(|x, y| x * y), "*".to_string()),
//         // UiuaElements::Div => (Box::new(|x, y| x / y), "/".to_string()),
//         _ => todo!(),
//     }
// }

// pub fn uiua_binary(a: UiuaElements, b: UiuaElements, f: UiuaElements) -> UiuaElements {
//     let (f, name) = get_binary(f);
//     match (a, b) {
//         (UiuaElements::Elem(lhs), UiuaElements::Elem(rhs)) => UiuaElements::Elem(f(lhs, rhs)),
//         (UiuaElements::Vector(v), UiuaElements::Elem(rhs)) => {
//             UiuaElements::Vector(v.iter().map(|x| f(*x, rhs)).collect())
//         }
//         (UiuaElements::Elem(lhs), UiuaElements::Vector(v)) => {
//             UiuaElements::Vector(v.iter().map(|x| f(lhs, *x)).collect())
//         }
//         (UiuaElements::Vector(lhs), UiuaElements::Vector(rhs)) => {
//             if lhs.len() != rhs.len() {
//                 return UiuaElements::Error(
//                     format!("Vector length mismatch: {:?} and {:?}", lhs, rhs).to_string(),
//                 );
//             }
//             UiuaElements::Vector(
//                 lhs.iter()
//                     .zip(rhs.into_iter())
//                     .map(|(x, y)| f(*x, y))
//                     .collect(),
//             )
//         }
//         (e1, e2) => UiuaElements::Error(format!(
//             "Arguments of {} cannot be of types {:?} and {:?}",
//             name, e1, e2
//         )),
//     }
// }

                    // UiuaElements::Plus
                    // | UiuaElements::Minus
                    // | UiuaElements::Mult
                    // | UiuaElements::Div => match reverse_stack.pop() {
                    //     Some(lhs) => match reverse_stack.pop() {
                    //         Some(rhs) => {
                    //             let res = uiua_binary(lhs, rhs, elem.clone());
                    //             reverse_stack.push(res);
                    //         }
                    //         None => {
                    //             return UiuaElements::Error(format!(
                    //                 "Not enough arguments for a function {:?}",
                    //                 elem.clone()
                    //             ))
                    //         }
                    //     },
                    //     None => {
                    //         return UiuaElements::Error(format!(
                    //             "Not enough arguments for a function {:?}",
                    //             elem.clone()
                    //         ))
                    //     }
                    // },

fn err() -> UiuaElements {
    UiuaElements::Error(format!("Not enough arguments for a function"))
        // "Not enough arguments for a function" {:?}",
        // d.clone()
   // ))
}

fn get_vars_or_err(mut stack: Vec<UiuaElements>) -> Result<(UiuaElements, UiuaElements), UiuaElements> {
    match stack.pop() {
        Some(lhs) => match stack.pop() {
            Some(rhs) => Ok((lhs, rhs)),
            None => Err(err())
        },
        None => Err(err())
    }
}

impl Performer for DyadicArithmetic {
    fn perform(&self, mut stack: Vec<UiuaElements>) -> Result<Vec<UiuaElements>, UiuaElements> {
        
        match &self {
            DyadicArithmetic::Add => { return Err(UiuaElements::Error("A".to_string()))}
            _ => {}
        }
        Ok(stack)
    }
}