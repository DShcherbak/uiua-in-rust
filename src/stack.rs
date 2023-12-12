use crate::elems::UiuaElements;

pub struct UiuaStack {
    pub chars: Vec<UiuaElements>,
}

pub fn get_binary(f: UiuaElements) -> (Box<dyn Fn(i32, i32) -> i32>, String) {
    match f {
        UiuaElements::Plus => (Box::new(|x, y| x + y), "+".to_string()),
        UiuaElements::Minus => (Box::new(|x, y| x - y), "-".to_string()),
        UiuaElements::Mult => (Box::new(|x, y| x * y), "*".to_string()),
        UiuaElements::Div => (Box::new(|x, y| x / y), "/".to_string()),
        _ => todo!(),
    }
}

pub fn uiua_binary(a: UiuaElements, b: UiuaElements, f: UiuaElements) -> UiuaElements {
    let (f, name) = get_binary(f);
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
                    .zip(rhs.into_iter())
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

impl UiuaStack {
    pub fn calc(&mut self) -> UiuaElements {
        loop {
            if self.chars.len() < 2 {
                break;
            }
            let mut reverse_stack: Vec<UiuaElements> = vec![];
            for elem in self.chars.iter() {
                match elem {
                    UiuaElements::Elem(x) => {
                        reverse_stack.push(UiuaElements::Elem(*x));
                    }
                    UiuaElements::Vector(x) => {
                        reverse_stack.push(UiuaElements::Vector(x.clone()));
                    }
                    UiuaElements::Plus
                    | UiuaElements::Minus
                    | UiuaElements::Mult
                    | UiuaElements::Div => match reverse_stack.pop() {
                        Some(lhs) => match reverse_stack.pop() {
                            Some(rhs) => {
                                let res = uiua_binary(lhs, rhs, elem.clone());
                                reverse_stack.push(res);
                            }
                            None => {
                                return UiuaElements::Error(format!(
                                    "Not enough arguments for a function {:?}",
                                    elem.clone()
                                ))
                            }
                        },
                        None => {
                            return UiuaElements::Error(format!(
                                "Not enough arguments for a function {:?}",
                                elem.clone()
                            ))
                        }
                    },
                    UiuaElements::Dot => match reverse_stack.pop() {
                        Some(rhs) => {
                            reverse_stack.push(rhs.clone());
                            reverse_stack.push(rhs);
                        }
                        None => {
                            return UiuaElements::Error(format!(
                                "Not enough arguments for a function {:?}",
                                elem.clone()
                            ))
                        }
                    },
                    UiuaElements::Comma => match reverse_stack.pop() {
                        Some(lhs) => match reverse_stack.pop() {
                            Some(rhs) => {
                                reverse_stack.push(rhs.clone());
                                reverse_stack.push(lhs);
                                reverse_stack.push(rhs);
                            }
                            None => {
                                return UiuaElements::Error(format!(
                                    "Not enough arguments for a function {:?}",
                                    elem.clone()
                                ))
                            }
                        },
                        None => {
                            return UiuaElements::Error(format!(
                                "Not enough arguments for a function {:?}",
                                elem.clone()
                            ))
                        }
                    },
                    UiuaElements::Semicolon => match reverse_stack.pop() {
                        Some(lhs) => {}
                        None => {
                            return UiuaElements::Error(format!(
                                "Not enough arguments for a function {:?}",
                                elem.clone()
                            ))
                        }
                    },
                    UiuaElements::DoubleColon => match reverse_stack.pop() {
                        Some(lhs) => match reverse_stack.pop() {
                            Some(rhs) => {
                                reverse_stack.push(lhs);
                                reverse_stack.push(rhs);
                            }
                            None => {
                                return UiuaElements::Error(format!(
                                    "Not enough arguments for a function {:?}",
                                    elem.clone()
                                ))
                            }
                        },
                        None => {
                            return UiuaElements::Error(format!(
                                "Not enough arguments for a function {:?}",
                                elem.clone()
                            ))
                        }
                    },
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
