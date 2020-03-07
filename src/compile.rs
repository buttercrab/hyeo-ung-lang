use std::collections::HashMap;

use crate::code::{Code, State};
use crate::execute::{pop_stack_wrap, push_stack_wrap};
use crate::number::Num;
use crate::{code, execute, io};
use std::io::Write;

fn opt_execute<T: code::State>(
    out: &mut impl Write,
    err: &mut impl Write,
    mut state: T,
    code: &T::CodeType,
) -> (T, bool) {
    let mut chk = true;
    let mut cur_loc = state.push_code((*code).clone());
    let length = cur_loc + 1;

    while cur_loc < length {
        let code = (*state.get_code(cur_loc)).clone();
        let mut cur_stack = state.current_stack();

        match code.get_type() {
            0 => {
                push_stack_wrap(
                    out,
                    err,
                    &mut state,
                    cur_stack,
                    &Num::from_num(code.get_hangul_count() as isize)
                        * &Num::from_num(code.get_dot_count() as isize),
                );
            }
            1 => {
                let mut n = Num::zero();
                for _ in 0..code.get_hangul_count() {
                    if cur_stack <= 2 {
                        chk = false;
                    }
                    cur_stack = 3;
                    n += &pop_stack_wrap(&mut state, cur_stack);
                }
                push_stack_wrap(out, err, &mut state, code.get_dot_count(), n);
            }
            2 => {
                let mut n = Num::one();
                for _ in 0..code.get_hangul_count() {
                    if cur_stack <= 2 {
                        chk = false;
                    }
                    cur_stack = 3;
                    n *= &pop_stack_wrap(&mut state, cur_stack);
                }
                push_stack_wrap(out, err, &mut state, code.get_dot_count(), n);
            }
            3 => {
                let mut n = Num::zero();
                let mut v = Vec::with_capacity(code.get_hangul_count());

                for _ in 0..code.get_hangul_count() {
                    if cur_stack <= 2 {
                        chk = false;
                    }
                    cur_stack = 3;
                    v.push(pop_stack_wrap(&mut state, cur_stack));
                }

                for mut x in v {
                    x.minus();
                    n += &x;
                    push_stack_wrap(out, err, &mut state, cur_stack, x);
                }

                push_stack_wrap(out, err, &mut state, code.get_dot_count(), n);
            }
            4 => {
                let mut n = Num::one();
                let mut v = Vec::with_capacity(code.get_hangul_count());

                for _ in 0..code.get_hangul_count() {
                    if cur_stack <= 2 {
                        chk = false;
                    }
                    cur_stack = 3;
                    v.push(pop_stack_wrap(&mut state, cur_stack));
                }

                for mut x in v {
                    x.flip();
                    n *= &x;
                    push_stack_wrap(out, err, &mut state, cur_stack, x);
                }

                push_stack_wrap(out, err, &mut state, code.get_dot_count(), n);
            }
            // 5
            _ => {
                if cur_stack == 0 {
                    chk = false;
                }
                let n = state.pop_stack(cur_stack);
                for _ in 0..code.get_hangul_count() {
                    state.push_stack(code.get_dot_count(), n.clone());
                }
                state.push_stack(cur_stack, n);
                state.set_current_stack(code.get_dot_count());
            }
        }

        cur_stack = state.current_stack();
        let area_type = code::calc(code.get_area(), code.get_area_count(), || {
            if cur_stack <= 2 {
                chk = false;
            }
            cur_stack = 3;
            pop_stack_wrap(&mut state, cur_stack)
        });

        if area_type != 0 {
            if area_type != 13 {
                let id = ((code.get_area_count() as u128) << 4) + area_type as u128;
                match state.get_point(id, cur_loc) {
                    Some(value) => {
                        if cur_loc != value {
                            cur_loc = value;
                            continue;
                        }
                    }
                    None => state.set_point(id, cur_loc),
                }
            } else {
                if let Some(loc) = state.get_latest_loc() {
                    cur_loc = loc;
                    continue;
                }
            }
        }

        cur_loc += 1;
    }

    (state, chk)
}

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

            let tup = opt_execute(&mut t_out, &mut t_err, opt_state, &opt_code);
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
