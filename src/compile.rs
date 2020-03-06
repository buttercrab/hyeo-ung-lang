use std::{error, fmt};

use crate::code;
use crate::io::print_error;

pub enum Error {
    LevelError(usize)
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::LevelError(level) => write!(f, "optimize level {} is not supported", level)
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::LevelError(level) => write!(f, "optimize level {} is not supported", level)
        }
    }
}

impl error::Error for Error {}

pub fn optimize(code: Vec<code::UnOptCode>, level: usize) -> (code::OptState, Vec<code::OptCode>) {
    let mut size = 0usize;
    let mut code = Vec::new();

    if level >= 3 {
        print_error(Error::LevelError(level))
    }

    if level >= 1 {
        // optimization level 1
    }

    let mut state = code::OptState::new(size);

    if level >= 2 {
        // optimization level 2
    }

    (state, code)
}

pub fn build_source<T: code::State>(state: T, code: &Vec<T::CodeType>) -> String {
    unimplemented!()
}