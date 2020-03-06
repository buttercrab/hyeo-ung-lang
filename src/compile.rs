use crate::code::Area;
use std::{error, fmt};

use crate::code;
use crate::code::Code;
use crate::io::print_error;
use std::collections::HashMap;

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
    let mut opt_code : Vec<code::OptCode> = Vec::new();

    if level >= 3 {
        print_error(Error::LevelError(level))
    }

    if level >= 1 {
        // optimization level 1
        let mut area_map : HashMap<usize,usize> = HashMap::new();
        let mut dot_map : HashMap<usize,usize> = HashMap::new();
        let mut max : usize = 1; 

        for unopt_code in &code {
            let mut cnt = area_map.entry(unopt_code.get_area_count()).or_insert(0);
            if *cnt == 0 {
                *cnt = max;
                max += 1;
            }
        }

        max = 4;
        for unopt_code in &code {
            if unopt_code.get_type() == 0 {
                continue;
            }
            if unopt_code.get_dot_count() <= 3 {
                continue;
            }

            let cnt = dot_map.entry(unopt_code.get_dot_count()).or_insert(0);
            if *cnt == 0 {
                *cnt = max;
                max += 1;
            }
        }

        for unopt_code in &code {
            let opt_type_ = unopt_code.get_type();
            let opt_hangul_count = unopt_code.get_hangul_count();
            let mut opt_dot_count = unopt_code.get_dot_count();
            let opt_area_count = *area_map.get(&(unopt_code.get_area_count())).unwrap();
            let opt_area = unopt_code.get_area().clone();

            if opt_type_ != 0 {
                if unopt_code.get_dot_count() > 3 {
                    opt_dot_count = *dot_map.get(&(unopt_code.get_dot_count())).unwrap();
                }
            } 
            
            opt_code.push(code::OptCode::new(
                opt_type_,
                opt_hangul_count,
                opt_dot_count,
                opt_area_count,
                opt_area
            ));
        }

        size = max ;
    }

    let mut state = code::OptState::new(size);
    if level >= 2 {
        // optimization level 2
        
    }

    (state, opt_code)
}

pub fn build_source<T: code::State>(state: T, code: &Vec<T::CodeType>) -> String {
    unimplemented!()
}