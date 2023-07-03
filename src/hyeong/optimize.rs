use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead, Read, Write};

use anyhow::Result;

use number::num::Num;

use crate::hyeong::code::{Code, HangulType, OptCode, UnOptCode};
use crate::hyeong::execute::ExecutableState;
use crate::hyeong::state::{OptState, State};

struct RedirectOutput(Vec<u8>);

impl Write for RedirectOutput {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl RedirectOutput {
    pub fn new() -> RedirectOutput {
        RedirectOutput(Vec::new())
    }

    pub fn to_string(&self) -> Result<String> {
        Ok(String::from_utf8(self.0.clone())?)
    }
}

struct NoInput;

impl Read for NoInput {
    fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> {
        Err(io::Error::new(
            io::ErrorKind::Other,
            anyhow::Error::msg("no-input"),
        ))
    }
}

impl BufRead for NoInput {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        Err(io::Error::new(
            io::ErrorKind::Other,
            anyhow::Error::msg("no-input"),
        ))
    }

    fn consume(&mut self, _amt: usize) {}
}

fn optimize1(code: Vec<UnOptCode>) -> Result<(OptState, Vec<OptCode>)> {
    let mut now = 3;
    let mut chk = HashSet::new();

    for c in &code {
        if matches!(c.hangul_type(), HangulType::Hyeong) {
            continue;
        }
        let _ = chk.insert(now);
        if matches!(c.hangul_type(), HangulType::Heuk) {
            now = c.dot_count();
        }
    }

    let dot_map = chk
        .into_iter()
        .filter(|x| *x <= 2)
        .enumerate()
        .map(|(i, x)| (x, i + 3))
        .collect::<HashMap<_, _>>();

    let opt_code = code
        .into_iter()
        .map(|c| {
            let type_ = c.hangul_type();
            let hangul_count = c.hangul_count();
            let dot_count = c.dot_count();
            let area = c.area().clone();
            let area_count = c.area_count();

            if matches!(c.hangul_type(), HangulType::Hyeong) || c.dot_count() <= 2 {
                OptCode::new(type_, hangul_count, dot_count, area_count, area)
            } else {
                let dot_count = dot_map.get(&dot_count).copied().unwrap_or(dot_map.len() + 3);
                OptCode::new(type_, hangul_count, dot_count, area_count, area)
            }
        })
        .collect::<Vec<_>>();

    Ok((OptState::new(dot_map.len() + 4), opt_code))
}

fn optimize2(mut state: OptState, code: Vec<OptCode>) -> Result<(OptState, Vec<OptCode>)> {
    let mut in_ = NoInput;
    let mut out = RedirectOutput::new();
    let mut err = RedirectOutput::new();

    let mut idx = code.len();
    for (i, opt_code) in code.iter().cloned().enumerate() {
        if let Err(e) = state.execute(&mut in_, &mut out, &mut err, opt_code) {
            if e.is::<usize>() {
                state.exit(e.downcast::<usize>().unwrap() as i32);
            } else if e.is::<io::Error>()
                && matches!(
                    e.downcast_ref::<io::Error>().unwrap().kind(),
                    io::ErrorKind::Other
                )
            {
                idx = i;
                break;
            } else {
                return Err(e);
            }
        }
    }
    let code = code[idx..].to_vec();

    state
        .get_stack(1)
        .extend(out.to_string()?.chars().map(|x| Num::from_num(x as isize)));
    state
        .get_stack(2)
        .extend(err.to_string()?.chars().map(|x| Num::from_num(x as isize)));
    Ok((state, code))
}

/// Optimization function
///
/// ## Level 1
///
/// In level 1, it analyzes the hyeong code and collect only used index of stack.
/// Then, re-number the indices not to make `HashMap`; using `Vec`
///
/// ## Level 2
///
/// In level 2, it runs code until it gets input, terminates, or too much loop.
/// So, `Hello, World!` code compiles to `print!("Hello, World!")` when unused code is removed.
///
/// # Examples
///
/// ```
/// use hyeong::core::{parse, optimize};
/// use hyeong::core::state::State;
///
/// let a = parse::parse(String::from("형... 항."));
/// let (mut s, c) = optimize::optimize(a, 2).unwrap();
///
/// assert_eq!("3", s.get_stack(1).iter().map(|x| x.to_string()).collect::<Vec<_>>().join(""))
/// ```
pub fn optimize(code: Vec<UnOptCode>, level: u8) -> Result<(OptState, Vec<OptCode>)> {
    let (state, code) = if level >= 1 {
        optimize1(code)?
    } else {
        (OptState::new(1), Vec::new())
    };

    if level >= 2 {
        optimize2(state, code)
    } else {
        Ok((state, code))
    }
}
