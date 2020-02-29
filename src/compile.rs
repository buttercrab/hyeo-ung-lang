use crate::{execute, parse};

pub struct Builder {}

impl Builder {
    pub fn new(state: execute::State) -> Builder {
        Builder {}
    }

    pub fn add(&self, code: parse::Command) {}
}

pub fn optimize(code: Vec<parse::Command>, level: usize) -> (execute::State, Vec<parse::Command>) {
    (execute::State::new(), vec![])
}