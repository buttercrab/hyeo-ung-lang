use crate::parse;

pub struct State {}

impl State {
    pub fn new() -> State {
        State {}
    }

    pub fn clone(&self) -> State {
        State {}
    }
}

pub fn execute(state: State, code: &parse::Command) -> State {
    State::new()
}