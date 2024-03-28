use crate::stack_manipulation::*;
use crate::elems::UiuaElements;
use crate::perform::Performer;

impl Performer for StackManipulation {
        fn perform(&self, mut stack: Vec<UiuaElements>) -> Result<Vec<UiuaElements>, UiuaElements> {
        match &self {
            StackManipulation::Duplicate => match stack.pop() {
                Some(rhs) => {
                    stack.push(rhs.clone());
                    stack.push(rhs);
                }
                None => {
                    return Err(UiuaElements::Error(format!(
                        "Not enough arguments for a function {:?}",
                        self
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
                            self
                        )))
                    }
                },
                None => {
                    return Err(UiuaElements::Error(format!(
                        "Not enough arguments for a function {:?}",
                        self
                    )))
                }
            },
            StackManipulation::Pop => match stack.pop() {
                Some(_) => {}
                None => {
                    return Err(UiuaElements::Error(format!(
                        "Not enough arguments for a function {:?}",
                        self
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
                        self
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
                            self
                        )))
                    }
                },
                None => {
                    return Err(UiuaElements::Error(format!(
                        "Not enough arguments for a function {:?}",
                        self
                    )))
                }
            },
        }
        Ok(stack)
    }
}