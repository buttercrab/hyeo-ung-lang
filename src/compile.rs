use crate::{code, execute};

pub struct Builder {}

impl Builder {
    pub fn new(state: execute::State) -> Builder {
        Builder {}
    }

    pub fn add(&self, code: code::UnOptCode) {}
}

pub fn optimize(code: Vec<code::UnOptCode>, level: usize) -> (execute::State, Vec<code::OptCode>) {
    (execute::State::new(), vec![])
}
