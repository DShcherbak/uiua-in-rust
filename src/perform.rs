use crate::UiuaElements;

pub trait Performer {
    fn perform(&self, stack: Vec<UiuaElements>) -> Result<Vec<UiuaElements>, UiuaElements>;
}