use crate::{execute, parse};

pub struct Builder {}

impl Builder {
    pub fn new(state: execute::State) -> Builder {
        Builder {}
    }

    pub fn add(&self, code: parse::UnOptCode) {}
}

pub fn optimize(code: Vec<parse::UnOptCode>, level: usize) -> (execute::State, Vec<parse::UnOptCode>) {
    (execute::State::new(), vec![])
}