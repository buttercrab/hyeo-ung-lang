use crate::code;

pub fn execute<T: code::State, U: code::Code>(state: &mut T, code: &U) {}