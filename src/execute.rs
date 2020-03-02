use crate::code;

pub struct State {}

impl State {
    pub fn new() -> State {
        State {}
    }

    pub fn clone(&self) -> State {
        State {}
    }
}

pub fn execute(state: &mut State, code: &dyn code::Code) {}