use std::collections::HashMap;

use crate::{code, number};

pub struct State {
    stacks: HashMap<usize, Vec<number::Num>>,
    code_stack: Vec<Box<dyn code::Code>>,
    break_points: HashMap<usize, usize>,
}

impl State {
    pub fn new() -> State {
        State {
            stacks: HashMap::new(),
            code_stack: vec![],
            break_points: HashMap::new(),
        }
    }
}

pub fn execute(state: &mut State, code: &dyn code::Code) {}