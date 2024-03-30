use std::rc::Rc;

use crate::{monadic_modifier::*};
use crate::elems::UiuaElements;

pub fn perform_modified(mut stack: Vec<UiuaElements>, oper: MonadicModifier, boxed: Rc<dyn Fn(i32, i32) -> i32 + 'static>) -> Result<Vec<UiuaElements>, UiuaElements> {
    match stack.pop() {
        Some(arg) => {
            match oper {
                MonadicModifier::Reduce => {
                    match arg {
                        UiuaElements::Vector(vec) => {
                            if vec.len() == 0 {
                                return Err(UiuaElements::Error(format!("Empty array in reduce operation")))
                            }
                            let first = *vec.get(0).unwrap();
                            let val = vec.iter().skip(1).fold(first, |x, &y| boxed(x, y));
                            stack.push(UiuaElements::Elem(val));
                            Ok(stack)
                        }
                        _ => Err(UiuaElements::Error(format!("Non-vector arg to reduce operation")))
                    }
                }
            }
        }
        None => Err(UiuaElements::Error(format!("Not enough arguments for a function {:?}", oper)))
    }
}
