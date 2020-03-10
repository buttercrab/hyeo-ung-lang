use std::collections::HashMap;

use crate::code::{Code, State};
use crate::execute::{pop_stack_wrap, push_stack_wrap};
use crate::io::ReadLine;
use crate::number::Num;
use crate::{code, io};
use std::io::{stdin, Write};

fn opt_execute<T>(
    ipt: &mut impl ReadLine,
    out: &mut impl Write,
    err: &mut impl Write,
    mut state: T,
    code: &T::CodeType,
) -> (T, bool)
where
    T: code::State + Clone,
{
    let mut cur_loc = state.push_code((*code).clone());
    let length = cur_loc + 1;
    let state_clone = state.clone();
    let mut exec_count = 0;

    while cur_loc < length {
        if exec_count >= 100 {
            return (state_clone, false);
        }

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
                        return (state_clone, false);
                    }
                    n += &pop_stack_wrap(ipt, out, err, &mut state, cur_stack);
                }
                push_stack_wrap(out, err, &mut state, code.get_dot_count(), n);
            }
            2 => {
                let mut n = Num::one();
                for _ in 0..code.get_hangul_count() {
                    if cur_stack <= 2 {
                        return (state_clone, false);
                    }
                    n *= &pop_stack_wrap(ipt, out, err, &mut state, cur_stack);
                }
                push_stack_wrap(out, err, &mut state, code.get_dot_count(), n);
            }
            3 => {
                let mut n = Num::zero();
                let mut v = Vec::with_capacity(code.get_hangul_count());

                for _ in 0..code.get_hangul_count() {
                    if cur_stack <= 2 {
                        return (state_clone, false);
                    }
                    v.push(pop_stack_wrap(ipt, out, err, &mut state, cur_stack));
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
                        return (state_clone, false);
                    }
                    v.push(pop_stack_wrap(ipt, out, err, &mut state, cur_stack));
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
                if cur_stack <= 2 {
                    return (state_clone, false);
                }
                let n = pop_stack_wrap(ipt, out, err, &mut state, cur_stack);
                for _ in 0..code.get_hangul_count() {
                    push_stack_wrap(out, err, &mut state, code.get_dot_count(), n.clone());
                }
                push_stack_wrap(out, err, &mut state, cur_stack, n);
                state.set_current_stack(code.get_dot_count());
            }
        }

        cur_stack = state.current_stack();
        let area_type = match code::calc(code.get_area(), code.get_area_count(), || {
            if cur_stack <= 2 {
                Option::None
            } else {
                Option::Some(pop_stack_wrap(ipt, out, err, &mut state, cur_stack))
            }
        }) {
            Some(value) => value,
            None => return (state_clone, false),
        };

        if area_type != 0 {
            if area_type != 13 {
                let id = ((code.get_area_count() as u128) << 4) + area_type as u128;
                match state.get_point(id) {
                    Some(value) => {
                        if cur_loc != value {
                            state.set_latest_loc(cur_loc);
                            cur_loc = value;
                            exec_count += 1;
                            continue;
                        }
                    }
                    None => state.set_point(id, cur_loc),
                }
            } else {
                if let Some(loc) = state.get_latest_loc() {
                    cur_loc = loc;
                    exec_count += 1;
                    continue;
                }
            }
        }

        cur_loc += 1;
    }

    (state, true)
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
        let mut now = 3;
        let mut chk = Vec::new();

        for un_opt_code in &code {
            if un_opt_code.get_type() == 0 {
                continue;
            }
            chk.push(now);
            if un_opt_code.get_type() == 5 {
                now = un_opt_code.get_dot_count();
            }
        }

        chk.sort();
        for i in chk {
            if i <= 3 {
                continue;
            }
            let temp = dot_map.entry(i).or_insert(0);
            if *temp == 0 {
                *temp = max;
                max += 1;
            }
        }

        for un_opt_code in &code {
            let opt_type_ = un_opt_code.get_type();
            let opt_hangul_count = un_opt_code.get_hangul_count();
            let mut opt_dot_count = un_opt_code.get_dot_count();
            let opt_area_count = un_opt_code.get_area_count();
            let opt_area = un_opt_code.get_area().clone();

            if opt_type_ == 0 || un_opt_code.get_dot_count() <= 3 {
                opt_code_vec.push(code::OptCode::new(
                    opt_type_,
                    opt_hangul_count,
                    opt_dot_count,
                    opt_area_count,
                    opt_area,
                ));
            } else {
                let temp = dot_map.entry(opt_dot_count).or_insert(0);
                if *temp == 0 {
                    *temp = max;
                }
                opt_dot_count = *temp;
                opt_code_vec.push(code::OptCode::new(
                    opt_type_,
                    opt_hangul_count,
                    opt_dot_count,
                    opt_area_count,
                    opt_area,
                ));
            }
        }

        size = max + 1;
    }

    let mut state = code::OptState::new(size);

    if level >= 2 {
        let mut out = io::CustomWriter::new(|_| Result::Ok(()));
        let mut err = io::CustomWriter::new(|_| Result::Ok(()));

        let mut idx = opt_code_vec.len();
        for (i, opt_code) in opt_code_vec.iter().enumerate() {
            let (new_state, next) = opt_execute(&mut stdin(), &mut out, &mut err, state, opt_code);
            state = new_state;
            if !next {
                idx = i;
                break;
            }
        }
        opt_code_vec = opt_code_vec[idx..].to_vec();

        state.get_stack(1).extend(
            out.to_string()
                .as_bytes()
                .iter()
                .map(|&x| Num::from_num(x as isize)),
        );
        state.get_stack(2).extend(
            err.to_string()
                .as_bytes()
                .iter()
                .map(|&x| Num::from_num(x as isize)),
        );
    }

    (state, opt_code_vec)
}
