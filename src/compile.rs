use crate::code;

pub fn optimize(code: Vec<code::UnOptCode>, level: usize) -> (code::OptState, Vec<code::OptCode>) {
    unimplemented!()
}

pub fn build_source<T: code::State>(state: T, code: &Vec<T::CodeType>) -> String {
    unimplemented!()
}