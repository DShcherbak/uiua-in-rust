use crate::stack::UiuaStack;
use crate::{elems::*, Performer};

impl UiuaStack {
    pub fn calc(&mut self) -> UiuaElements {
        if self.chars.len() >= 2 {
            let mut reverse_stack: Vec<UiuaElements> = vec![];
            for elem in self.chars.iter() {
                print!("Stack: {:?}", self);
                print!("Reversed stack: {:?}", reverse_stack);
                match elem {
                    UiuaElements::Elem(x) => {
                        reverse_stack.push(UiuaElements::Elem(*x));
                    }
                    UiuaElements::Vector(x) => {
                        reverse_stack.push(UiuaElements::Vector(x.clone()));
                    }
                    UiuaElements::Operator(UiuaOperator::StackManipulation(oper)) => {
                        match oper.perform(reverse_stack) {
                            Err(e) => { return e; }
                            Ok(stack) => {reverse_stack = stack;}
                        }
                    }
                    UiuaElements::Operator(UiuaOperator::DyadicArithmetic(oper)) => {
                        match oper.perform(reverse_stack) {
                            Err(e) => { return e; }
                            Ok(stack) => {reverse_stack = stack;}
                        }
                    }                    
                    UiuaElements::Error(msg) => return UiuaElements::Error(msg.clone()),
                    _ => todo!()
                }
            }
            self.chars = reverse_stack.iter().rev().cloned().collect();
        }
        match self.chars.get(0) {
            Some(val) => val.clone(),
            None => UiuaElements::Elem(0),
        }
    }
}
