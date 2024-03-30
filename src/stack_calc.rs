use crate::perform_dyadic_arith::get_dyadic_function;
use crate::perform_monadic_modifier::perform_modified;
use crate::stack::UiuaStack;
use crate::{elems::*, Performer};

impl UiuaStack {
    fn precalc(&mut self) -> Option<UiuaElements>{
        let mut next = false;
        let mut applied_id = 0;
        let mut new_stack: Vec<UiuaElements> = vec![];
        for elem in self.chars.iter().rev() {
            if let UiuaElements::Operator(UiuaOperator::MonadicModifier(modif)) = elem {
                new_stack.push(UiuaElements::Operator(UiuaOperator::AppliedMonadicModifier(modif.clone(), applied_id)));
                next = true;
                continue;
            }
            if next {
                if let UiuaElements::Operator(UiuaOperator::DyadicArithmetic(oper)) = elem {
                    self.applied.insert(applied_id, get_dyadic_function(oper).0);
                    applied_id += 1;
                    next = false;
                } else {
                    return Some(UiuaElements::Error(format!("Some operator was not applied")));
                }
                continue;
            }
            new_stack.push(elem.clone());
        }
        if next {
            return Some(UiuaElements::Error(format!("Some operator was not applied")));
        }
        new_stack.reverse();
        self.chars = new_stack;
        None
    }

    pub fn calc(&mut self) -> UiuaElements {
        if let Some(err) = self.precalc() {
            return err;
        }
        if self.chars.len() >= 2 {
            let mut reverse_stack: Vec<UiuaElements> = vec![];
            for elem in self.chars.iter() {
              //  println!("Stack: {:?}", self);
               // println!("Reversed stack: {:?}", reverse_stack);
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
                    UiuaElements::Operator(UiuaOperator::MonadicAriphmethic) => {
                        todo!()
                    }
                    UiuaElements::Operator(UiuaOperator::DyadicArithmetic(oper)) => {
                        match oper.perform(reverse_stack) {
                            Err(e) => { return e; }
                            Ok(stack) => {reverse_stack = stack;}
                        }
                    }
                    UiuaElements::Operator(UiuaOperator::MonadicModifier(oper)) => {
                        return UiuaElements::Error(format!("Unapplyied monadic modifier {:?}", oper));
                    }
                    UiuaElements::Operator(UiuaOperator::AppliedMonadicModifier(oper, id)) => { 
                        if let Some(func) = self.applied.get(id) {
                            match perform_modified(reverse_stack, oper.clone(), func.clone()) {
                                Err(e) => { return e; }
                                Ok(stack) => {reverse_stack = stack;}
                            }
                        }
                    }
                    UiuaElements::Operator(UiuaOperator::DyadicModifier) => {
                        todo!()
                    }
                    UiuaElements::Operator(UiuaOperator::MonadicArray(oper)) => {
                        match oper.perform(reverse_stack) {
                            Err(e) => { return e; }
                            Ok(stack) => {reverse_stack = stack;}
                        }
                    }
                    UiuaElements::Operator(UiuaOperator::DyadicArray) => {
                        todo!()
                    }
                    UiuaElements::Error(msg) => return UiuaElements::Error(msg.clone()),
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
