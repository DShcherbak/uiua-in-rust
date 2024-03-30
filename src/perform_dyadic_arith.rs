use std::rc::Rc;

use crate::{dyadic_arith::*, Performer};
use crate::elems::UiuaElements;

pub fn get_dyadic_function(f: &DyadicArithmetic) -> (Rc<dyn Fn(i32, i32) -> i32>, String) {
    match f {
        DyadicArithmetic::Add => (Rc::new(|x, y| x + y), "+".to_string()),
        DyadicArithmetic::Sub => (Rc::new(|x, y| x - y), "-".to_string()),
        DyadicArithmetic::Mult => (Rc::new(|x, y| x * y), "*".to_string()),
        DyadicArithmetic::Div => (Rc::new(|x, y| x / y), "÷".to_string()),
        DyadicArithmetic::Eq => (Rc::new(|x, y| {if x == y { 1 } else {0} } ), "=".to_string()),
        DyadicArithmetic::Neq => (Rc::new(|x, y| { if x == y { 0 } else {1} }), "≠".to_string()),
        DyadicArithmetic::LT => (Rc::new(|x, y| { if x < y { 1 } else {0} }), "<".to_string()),
        DyadicArithmetic::GT => (Rc::new(|x, y| { if x > y { 1 } else {0} }), ">".to_string()),
        DyadicArithmetic::LET => (Rc::new(|x, y| { if x >= y { 1 } else {0} }), "≤".to_string()),
        DyadicArithmetic::GET => (Rc::new(|x, y| {  if x <= y { 1 } else {0}}), "≥".to_string()),
        DyadicArithmetic::Mod => (Rc::new(|x, y| { y % x }), "◿".to_string()),
        DyadicArithmetic::Exp => (Rc::new(|x, y| { y.pow(x.try_into().unwrap()) }), "ⁿ".to_string()),
      //  DyadicArithmetic::Log => (Rc::new(|x, y| { log }), "ₙ".to_string()),
        DyadicArithmetic::Min => (Rc::new(|x, y| { if x < y { x } else { y } }), "↧".to_string()),
        DyadicArithmetic::Max => (Rc::new(|x, y| { if x > y { x } else { y } }), "↥".to_string()),
        //DyadicArithmetic::Atan => (Rc::new(|x, y| { x }), "∠".to_string()),

    }
}

fn apply_uiua_binary(a: UiuaElements, b: UiuaElements, f: &DyadicArithmetic) -> UiuaElements {
    let (f, name) = get_dyadic_function(f);
    match (a, b) {
        (UiuaElements::Elem(lhs), UiuaElements::Elem(rhs)) => UiuaElements::Elem(f(lhs, rhs)),
        (UiuaElements::Vector(v), UiuaElements::Elem(rhs)) => {
            UiuaElements::Vector(v.iter().map(|x| f(*x, rhs)).collect())
        }
        (UiuaElements::Elem(lhs), UiuaElements::Vector(v)) => {
            UiuaElements::Vector(v.iter().map(|x| f(lhs, *x)).collect())
        }
        (UiuaElements::Vector(lhs), UiuaElements::Vector(rhs)) => {
            if lhs.len() != rhs.len() {
                return UiuaElements::Error(
                    format!("Vector length mismatch: {:?} and {:?}", lhs, rhs).to_string(),
                );
            }
            UiuaElements::Vector(
                lhs.iter()
                    .zip(rhs)
                    .map(|(x, y)| f(*x, y))
                    .collect(),
            )
        }
        (e1, e2) => UiuaElements::Error(format!(
            "Arguments of {} cannot be of types {:?} and {:?}",
            name, e1, e2
        )),
    }
}

fn err(oper: &DyadicArithmetic) -> UiuaElements {
    UiuaElements::Error(format!(
        "Not enough arguments for a function {:?}",
        oper
   ))
}

fn get_vars_or_err(oper: &DyadicArithmetic, stack: &mut Vec<UiuaElements>) -> Result<(UiuaElements, UiuaElements), UiuaElements> {
    match stack.pop() {
        Some(lhs) => match stack.pop() {
            Some(rhs) => Ok((lhs, rhs)),
            None => Err(err(oper))
        },
        None => Err(err(oper))
    }
}

impl Performer for DyadicArithmetic {
    fn perform(&self, mut stack: Vec<UiuaElements>) -> Result<Vec<UiuaElements>, UiuaElements> {
        match get_vars_or_err(self, &mut stack) {
            Ok((lhs, rhs)) => {
                stack.push(apply_uiua_binary(lhs, rhs, self));
                Ok(stack)
            }
            Err(e) => Err(e)
        }
    }
}