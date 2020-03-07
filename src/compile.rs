use std::collections::HashMap;

use crate::code::{Code, State};
use crate::number::Num;
use crate::{code, execute, io};

pub fn optimize(code: Vec<code::UnOptCode>, level: usize) -> (code::OptState, Vec<code::OptCode>) {
    let mut size = 0usize;
    let mut opt_code_vec: Vec<code::OptCode> = Vec::new();

    if level >= 3 {
        io::print_error_string(&*format!("optimize level {} is not supported", level));
    }

    io::print_log(&*format!("optimizing to level {}", level));

    if level >= 1 {
        let mut dot_map: HashMap<usize, usize> = HashMap::new();
        let mut max: usize = 4;

        for un_opt_code in &code {
            if un_opt_code.get_type() == 0 {
                continue;
            }
            if un_opt_code.get_dot_count() <= 3 {
                continue;
            }

            let cnt = dot_map.entry(un_opt_code.get_dot_count()).or_insert(0);
            if *cnt == 0 {
                *cnt = max;
                max += 1;
            }
        }

        for un_opt_code in &code {
            let opt_type_ = un_opt_code.get_type();
            let opt_hangul_count = un_opt_code.get_hangul_count();
            let mut opt_dot_count = un_opt_code.get_dot_count();
            let opt_area_count = un_opt_code.get_area_count();
            let opt_area = un_opt_code.get_area().clone();

            if opt_type_ != 0 {
                if un_opt_code.get_dot_count() > 3 {
                    opt_dot_count = *dot_map.get(&(un_opt_code.get_dot_count())).unwrap();
                }
            }

            opt_code_vec.push(code::OptCode::new(
                opt_type_,
                opt_hangul_count,
                opt_dot_count,
                opt_area_count,
                opt_area,
            ));
        }

        size = max;
    }

    let mut state = code::OptState::new(size);

    if level >= 2 {
        let mut opt_state = code::OptState::new(size);
        let mut temp_vec: Vec<code::OptCode> = Vec::new();
        let mut out = io::CustomWriter::new();
        let mut err = io::CustomWriter::new();
        let mut t_out = io::CustomWriter::new();
        let mut t_err = io::CustomWriter::new();
        let mut chk = true;
        
        for opt_code in &opt_code_vec {
            if !chk {
                temp_vec.push(opt_code.clone());
                continue;
            } 

            let tup = execute::opt_execute(&mut t_out, &mut t_err, opt_state, &opt_code);
            opt_state = tup.0;

            if tup.1 {
                state = execute::execute(&mut out, &mut err, state, &opt_code);
            } else {
                chk = false;
                temp_vec.push(opt_code.clone());
            }
        }
        opt_code_vec = temp_vec;
        let out_str: String = out.to_string();
        let err_str: String = err.to_string();
        let mut out_vec = out_str
            .as_bytes()
            .iter()
            .map(|&x| Num::from_num(x as isize))
            .collect::<Vec<Num>>();
        let mut err_vec = err_str
            .as_bytes()
            .iter()
            .map(|&x| Num::from_num(x as isize))
            .collect::<Vec<Num>>();

        state.get_stack(1).append(&mut out_vec);
        state.get_stack(2).append(&mut err_vec);
    }

    (state, opt_code_vec)
}

pub fn build_source<T: code::State>(state: T, code: &Vec<T::CodeType>) -> String {
    unimplemented!()
}
