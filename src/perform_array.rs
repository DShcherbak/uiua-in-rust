use crate::{array::*, Performer};
use crate::elems::UiuaElements;

impl Performer for MonadicArray {
    fn perform(&self, mut stack: Vec<UiuaElements>) -> Result<Vec<UiuaElements>, UiuaElements> {
        match stack.pop() {
            Some(arg) => {
                match &self {
                    MonadicArray::Len => {
                        match arg {
                            UiuaElements::Vector(vec) => {
                                stack.push(UiuaElements::Elem(vec.len() as i32));
                                Ok(stack)
                            }
                            _ => Err(UiuaElements::Error(format!("Non-vector arg to length operation")))
                        }
                    }
                    MonadicArray::Iota => {
                        match arg {
                            UiuaElements::Elem(val) => {
                                stack.push(UiuaElements::Vector((0..val).collect()));
                                Ok(stack)
                            }
                            _ => Err(UiuaElements::Error(format!("Non-scalar arg to range")))
                        }
                    }
                }
            }
            None => Err(UiuaElements::Error(format!("Not enough arguments for a function {:?}",&self)))
        }
    }
}