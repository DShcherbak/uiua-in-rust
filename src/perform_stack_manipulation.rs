use crate::stack_manipulation::*;
use crate::elems::UiuaElements;

pub fn perform(elem: UiuaElements, oper : StackManipulation, mut stack: Vec<UiuaElements>) -> Result<Vec<UiuaElements>, UiuaElements> {
    match oper {
        StackManipulation::Duplicate => match stack.pop() {
            Some(rhs) => {
                stack.push(rhs.clone());
                stack.push(rhs);
            }
            None => {
                return Err(UiuaElements::Error(format!(
                    "Not enough arguments for a function {:?}",
                    elem.clone()
                )))
            }
        },
        StackManipulation::Over => match stack.pop() {
            Some(lhs) => match stack.pop() {
                Some(rhs) => {
                    stack.push(rhs.clone());
                    stack.push(lhs);
                    stack.push(rhs);
                }
                None => {
                    return Err(UiuaElements::Error(format!(
                        "Not enough arguments for a function {:?}",
                        elem.clone()
                    )))
                }
            },
            None => {
                return Err(UiuaElements::Error(format!(
                    "Not enough arguments for a function {:?}",
                    elem.clone()
                )))
            }
        },
        StackManipulation::Pop => match stack.pop() {
            Some(_) => {}
            None => {
                return Err(UiuaElements::Error(format!(
                    "Not enough arguments for a function {:?}",
                    elem.clone()
                )))
            }
        },
        StackManipulation::Id => match stack.pop() {
            Some(lhs) => {
                stack.push(lhs);
            }
            None => {
                return Err(UiuaElements::Error(format!(
                    "Not enough arguments for a function {:?}",
                    elem.clone()
                )))
            }
        },
        StackManipulation::Flip => match stack.pop() {
            Some(lhs) => match stack.pop() {
                Some(rhs) => {
                    stack.push(lhs);
                    stack.push(rhs);
                }
                None => {
                    return Err(UiuaElements::Error(format!(
                        "Not enough arguments for a function {:?}",
                        elem.clone()
                    )))
                }
            },
            None => {
                return Err(UiuaElements::Error(format!(
                    "Not enough arguments for a function {:?}",
                    elem.clone()
                )))
            }
        },
    }
    Ok(stack)
}