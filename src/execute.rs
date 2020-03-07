use std::io::Write;
use std::process;

use crate::code::Code;
use crate::number::Num;
use crate::{code, io};

pub fn push_stack_wrap<T: code::State>(
    out: &mut impl Write,
    err: &mut impl Write,
    state: &mut T,
    idx: usize,
    num: Num,
) {
    match idx {
        1 => {
            if num.is_pos() {
                io::write(out, &*format!("{}", num.floor().to_int() as u8 as char));
            } else {
                io::write(out, &*format!("{}", -&num));
            }
            io::handle_error(out.flush());
        }
        2 => {
            if num.is_pos() {
                io::write(err, &*format!("{}", num.floor().to_int() as u8 as char));
            } else {
                io::write(err, &*format!("{}", -&num));
            }
            io::handle_error(err.flush());
        }
        _ => {
            state.push_stack(idx, num);
        }
    }
}

pub fn pop_stack_wrap<T: code::State>(state: &mut T, idx: usize) -> Num {
    match idx {
        0 => {
            if state.get_stack(0).is_empty() {
                let s = io::read_line();
                for c in s.chars().rev() {
                    state.push_stack(0, Num::from_num(c as isize));
                }
            }
            state.pop_stack(0)
        }
        1 => {
            process::exit(0);
        }
        2 => {
            process::exit(1);
        }
        _ => state.pop_stack(idx),
    }
}

pub fn execute_one<T: code::State>(out: &mut impl Write, err: &mut impl Write, mut state: T, cur_loc: usize) -> (T, usize) {
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
                n += &pop_stack_wrap(&mut state, cur_stack);
            }
            push_stack_wrap(out, err, &mut state, code.get_dot_count(), n);
        }
        2 => {
            let mut n = Num::one();
            for _ in 0..code.get_hangul_count() {
                n *= &pop_stack_wrap(&mut state, cur_stack);
            }
            push_stack_wrap(out, err, &mut state, code.get_dot_count(), n);
        }
        3 => {
            let mut n = Num::zero();
            let mut v = Vec::with_capacity(code.get_hangul_count());

            for _ in 0..code.get_hangul_count() {
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
            let n = pop_stack_wrap(&mut state, cur_stack);
            for _ in 0..code.get_hangul_count() {
                push_stack_wrap(out, err, &mut state, code.get_dot_count(), n.clone());
            }
            push_stack_wrap(out, err, &mut state, cur_stack, n);
            state.set_current_stack(code.get_dot_count());
        }
    }

    cur_stack = state.current_stack();
    let area_type = code::calc(code.get_area(), code.get_area_count(), || {
        Option::Some(pop_stack_wrap(&mut state, cur_stack))
    })
        .unwrap();

    if area_type != 0 {
        if area_type != 13 {
            let id = ((code.get_area_count() as u128) << 4) + area_type as u128;
            match state.get_point(id, cur_loc) {
                Some(value) => {
                    if cur_loc != value {
                        return (state, value);
                    }
                }
                None => state.set_point(id, cur_loc),
            }
        } else {
            if let Some(loc) = state.get_latest_loc() {
                return (state, loc);
            }
        }
    }

    (state, cur_loc + 1)
}

pub fn execute<T: code::State>(
    out: &mut impl Write,
    err: &mut impl Write,
    mut state: T,
    code: &T::CodeType,
) -> T {
    let mut cur_loc = state.push_code((*code).clone());
    let length = cur_loc + 1;

    while cur_loc < length {
        let (new_state, new_loc) = execute_one(out, err, state, cur_loc);
        state = new_state;
        cur_loc = new_loc;
    }

    state
}
