use std::cmp::min;

use crate::code;
use crate::code::Code;
use crate::number::Num;

pub fn execute<T: code::State>(mut state: T, code: &T::CodeType) -> T {
    let mut cur_loc = state.push_code((*code).clone());
    let length = cur_loc + 1;

    while cur_loc < length {
        let code = (*state.get_code(cur_loc)).clone();
        let cur = state.current_stack();

        match code.get_type() {
            0 => {
                state.push_stack(
                    cur,
                    &Num::from_num(code.get_hangul_count() as isize)
                        * &Num::from_num(code.get_dot_count() as isize),
                );
            }
            1 => {
                let mut n = Num::zero();
                for _ in 0..code.get_hangul_count() {
                    n += &state.pop_stack(cur);
                }
                state.push_stack(code.get_dot_count(), n);
            }
            2 => {
                let mut n = Num::one();
                for _ in 0..code.get_hangul_count() {
                    n *= &state.pop_stack(cur);
                }
                state.push_stack(code.get_dot_count(), n);
            }
            3 => {
                let mut n = Num::zero();
                let stack = state.get_stack(cur);

                for i in 0..min(stack.len(), code.get_hangul_count()) {
                    stack[i].minus();
                    n += &stack[i];
                }

                state.push_stack(code.get_dot_count(), n);
            }
            4 => {
                let mut n = Num::zero();
                let stack = state.get_stack(cur);

                for i in 0..min(stack.len(), code.get_hangul_count()) {
                    stack[i].flip();
                    n *= &stack[i];
                }

                state.push_stack(code.get_dot_count(), n);
            }
            5 => {
                let n = state.pop_stack(cur);
                for _ in 0..code.get_hangul_count() {
                    state.push_stack(code.get_dot_count(), n.clone());
                }
                state.push_stack(cur, n);
                state.set_current_stack(code.get_dot_count());
            }
            _ => unreachable!(),
        }

        cur_loc += 1;
    }

    state
}