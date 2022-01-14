use crate::core::state::{State, UnOptState};
use crate::core::{execute, optimize};
use crate::util::error::Error;
use crate::util::option::HyeongOption;
use crate::util::{ext, io, option};
use clap::App;
use std::io::Write;
use termcolor::StandardStream;

/// App for run
#[cfg(not(tarpaulin_include))]
pub fn app<'a>() -> App<'a> {
    App::new("run")
        .about("Run hyeong code directly")
        .arg(option::input())
        .arg(option::optimize())
}

/// App for run
///
/// 1. parse code
/// 2. optimize code
/// 3. execute code
#[cfg(not(tarpaulin_include))]
pub fn run(
    stdout: &mut StandardStream,
    stderr: &mut StandardStream,
    hy_opt: &HyeongOption,
) -> Result<(), Error> {
    let un_opt_code = ext::parse_file(stdout, hy_opt.input.as_ref().unwrap(), hy_opt)?;

    if hy_opt.optimize >= 1 {
        io::print_log(stdout, format!("optimizing to level {}", hy_opt.optimize))?;
        let (mut state, opt_code) = optimize::optimize(un_opt_code, hy_opt.optimize)?;
        io::print_log(stdout, "running code")?;

        if !state.get_stack(1).is_empty() {
            for num in state.get_stack(1).iter() {
                write!(stdout, "{}", ext::num_to_unicode(num)?)?;
            }
            stdout.flush()?;
            state.get_stack(1).clear();
        }

        if !state.get_stack(2).is_empty() {
            for num in state.get_stack(2).iter() {
                write!(stderr, "{}", ext::num_to_unicode(num)?)?;
            }
            stdout.flush()?;
            state.get_stack(2).clear();
        }

        for c in opt_code {
            state = execute::execute(&mut std::io::stdin(), stdout, stderr, state, &c)?;
        }
    } else {
        let mut state = UnOptState::new();
        io::print_log(stdout, "running code")?;

        for c in un_opt_code {
            state = execute::execute(&mut std::io::stdin(), stdout, stderr, state, &c)?;
        }
    };

    Ok(())
}
