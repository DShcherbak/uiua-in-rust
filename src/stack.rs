use std::{collections::HashMap, rc::Rc};

use crate::elems::UiuaElements;

pub struct UiuaStack {
    pub chars: Vec<UiuaElements>,
    pub applied : HashMap<usize, Rc<dyn Fn(i32, i32) -> i32 + 'static>>
}
