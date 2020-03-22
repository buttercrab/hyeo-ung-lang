use crate::core::area;
use crate::core::code::Code;
use crate::core::state::State;
use crate::number::number::Num;
use crate::util::error::Error;
use crate::util::io;
use crate::util::io::ReadLine;
use std::io::Write;
use std::process;

/// Wrapper function for pushing to stack
/// This is needed because stack no 1, 2 has different behavior
///
/// # Examples
///
/// ```
/// use hyeong::util::io::CustomWriter;
/// use hyeong::core::state::UnOptState;
/// use hyeong::core::execute;
/// use hyeong::number::number::Num;
///
/// let mut a = CustomWriter::new(|_| Result::Ok(()));
/// let mut b = CustomWriter::new(|_| Result::Ok(()));
/// let mut s = UnOptState::new();
///
/// execute::push_stack_wrap(&mut a, &mut b, &mut s, 1, Num::from_num(-1)).unwrap();
///
/// assert_eq!("1", a.to_string().unwrap());
/// ```
pub fn push_stack_wrap<T>(
    out: &mut impl Write,
    err: &mut impl Write,
    state: &mut T,
    idx: usize,
    num: Num,
) -> Result<(), Error>
where
    T: State,
{
    match idx {
        1 => {
            if num.is_pos() {
                write!(out, "{}", num.floor().to_int() as u8 as char)?;
            } else {
                write!(out, "{}", -&num)?;
            }
        }
        2 => {
            if num.is_pos() {
                write!(err, "{}", num.floor().to_int() as u8 as char)?;
            } else {
                write!(err, "{}", -&num)?;
            }
        }
        _ => state.push_stack(idx, num),
    }
    Ok(())
}

/// Wrapper function for popping from stack
/// This is needed because stack no 0, 1, 2 has different behavior
///
/// # Examples
///
/// ```
/// use hyeong::util::io::{CustomReader, CustomWriter};
/// use hyeong::core::state::UnOptState;
/// use hyeong::core::execute;
///
/// let mut a = CustomReader::new("0".to_string());
/// let mut b = CustomWriter::new(|_| Result::Ok(()));
/// let mut c = CustomWriter::new(|_| Result::Ok(()));
/// let mut s = UnOptState::new();
///
/// let n = execute::pop_stack_wrap(&mut a, &mut b, &mut c, &mut s, 0).unwrap();
/// assert_eq!("48", n.to_string());
/// ```
pub fn pop_stack_wrap<T>(
    ipt: &mut impl ReadLine,
    out: &mut impl Write,
    err: &mut impl Write,
    state: &mut T,
    idx: usize,
) -> Result<Num, Error>
where
    T: State,
{
    match idx {
        0 => {
            if state.get_stack(0).is_empty() {
                let s = io::read_line_from(ipt)?;
                for c in s.chars().rev() {
                    state.push_stack(0, Num::from_num(c as isize));
                }
            }
            Ok(state.pop_stack(0))
        }
        1 => {
            out.flush().unwrap();
            err.flush().unwrap();
            process::exit(0);
        }
        2 => {
            out.flush().unwrap();
            err.flush().unwrap();
            process::exit(1);
        }
        _ => Ok(state.pop_stack(idx)),
    }
}

/// Executes only one line of code and return next position of code
///
/// # Examples
///
/// ```
/// use hyeong::util::io::{CustomReader, CustomWriter};
/// use hyeong::core::state::{UnOptState, State};
/// use hyeong::core::{parse, execute};
///
/// let mut a = CustomReader::new("0".to_string());
/// let mut b = CustomWriter::new(|_| Result::Ok(()));
/// let mut c = CustomWriter::new(|_| Result::Ok(()));
/// let mut s = UnOptState::new();
/// let t = parse::parse("형...".to_string());
/// s.push_code(t[0].clone());
///
/// let (mut s, _) = execute::execute_one(&mut a, &mut b, &mut c, s, 0).unwrap();
/// assert_eq!("3", s.get_stack(3)[0].to_string());
/// ```
pub fn execute_one<T>(
    ipt: &mut impl ReadLine,
    out: &mut impl Write,
    err: &mut impl Write,
    mut state: T,
    cur_loc: usize,
) -> Result<(T, usize), Error>
where
    T: State,
{
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
            )?;
        }
        1 => {
            let mut n = Num::zero();
            for _ in 0..code.get_hangul_count() {
                n += &pop_stack_wrap(ipt, out, err, &mut state, cur_stack)?;
            }
            push_stack_wrap(out, err, &mut state, code.get_dot_count(), n)?;
        }
        2 => {
            let mut n = Num::one();
            for _ in 0..code.get_hangul_count() {
                n *= &pop_stack_wrap(ipt, out, err, &mut state, cur_stack)?;
            }
            push_stack_wrap(out, err, &mut state, code.get_dot_count(), n)?;
        }
        3 => {
            let mut n = Num::zero();
            let mut v = Vec::with_capacity(code.get_hangul_count());

            for _ in 0..code.get_hangul_count() {
                v.push(pop_stack_wrap(ipt, out, err, &mut state, cur_stack)?);
            }

            for mut x in v {
                x.minus();
                n += &x;
                push_stack_wrap(out, err, &mut state, cur_stack, x)?;
            }

            push_stack_wrap(out, err, &mut state, code.get_dot_count(), n)?;
        }
        4 => {
            let mut n = Num::one();
            let mut v = Vec::with_capacity(code.get_hangul_count());

            for _ in 0..code.get_hangul_count() {
                v.push(pop_stack_wrap(ipt, out, err, &mut state, cur_stack)?);
            }

            for mut x in v {
                x.flip();
                n *= &x;
                push_stack_wrap(out, err, &mut state, cur_stack, x)?;
            }

            push_stack_wrap(out, err, &mut state, code.get_dot_count(), n)?;
        }
        // 5
        _ => {
            let n = pop_stack_wrap(ipt, out, err, &mut state, cur_stack)?;
            for _ in 0..code.get_hangul_count() {
                push_stack_wrap(out, err, &mut state, code.get_dot_count(), n.clone())?;
            }
            push_stack_wrap(out, err, &mut state, cur_stack, n)?;
            state.set_current_stack(code.get_dot_count());
        }
    }

    cur_stack = state.current_stack();
    let area_type = area::calc(code.get_area(), code.get_area_count(), || {
        pop_stack_wrap(ipt, out, err, &mut state, cur_stack)
    })?;

    if area_type != 0 {
        if area_type != 13 {
            let id = ((code.get_area_count() as u128) << 4) + area_type as u128;
            match state.get_point(id) {
                Some(value) => {
                    if cur_loc != value {
                        state.set_latest_loc(cur_loc);
                        return Ok((state, value));
                    }
                }
                None => state.set_point(id, cur_loc),
            }
        } else {
            if let Some(loc) = state.get_latest_loc() {
                return Ok((state, loc));
            }
        }
    }

    Ok((state, cur_loc + 1))
}

/// Execute from new code until needs new code or finish
///
/// # Examples
///
/// ```
/// use hyeong::core::{execute, parse};
/// use hyeong::util::io::{CustomReader, CustomWriter};
/// use hyeong::core::state::{UnOptState, State};
///
/// let mut a = CustomReader::new("0".to_string());
/// let mut b = CustomWriter::new(|_| Result::Ok(()));
/// let mut c = CustomWriter::new(|_| Result::Ok(()));
/// let mut s = UnOptState::new();
/// let t = parse::parse("형...".to_string());
///
/// let mut s = execute::execute(&mut a, &mut b, &mut c, s, &t[0]).unwrap();
/// assert_eq!("3", s.get_stack(3)[0].to_string());
/// ```
pub fn execute<T>(
    ipt: &mut impl ReadLine,
    out: &mut impl Write,
    err: &mut impl Write,
    mut state: T,
    code: &T::CodeType,
) -> Result<T, Error>
where
    T: State,
{
    let mut cur_loc = state.push_code((*code).clone());
    let length = cur_loc + 1;

    while cur_loc < length {
        let (new_state, new_loc) = execute_one(ipt, out, err, state, cur_loc)?;
        state = new_state;
        cur_loc = new_loc;
    }

    Ok(state)
}
