use crate::core::state::{State, UnOptState};
use crate::core::{execute, optimize};
use crate::util::error::Error;
use crate::util::option::HyeongOption;
use crate::util::{io, option};
use clap::App;
use std::io::Write;
use termcolor::StandardStream;

#[cfg_attr(tarpaulin, skip)]
pub fn app<'a, 'b>() -> App<'a, 'b> {
    App::new("run")
        .about("Run hyeong code directly")
        .arg(option::input())
        .arg(option::optimize())
}

#[cfg_attr(tarpaulin, skip)]
pub fn run(
    stdout: &mut StandardStream,
    stderr: &mut StandardStream,
    hy_opt: HyeongOption,
) -> Result<(), Error> {
    let un_opt_code = io::parse_file(stdout, &hy_opt.input.unwrap())?;

    if hy_opt.optimize >= 1 {
        io::print_log(stdout, format!("optimizing to level {}", hy_opt.optimize))?;
        let (mut state, opt_code) = optimize::optimize(un_opt_code, hy_opt.optimize)?;
        io::print_log(stdout, "running code")?;

        if !state.get_stack(1).is_empty() {
            for num in state.get_stack(1).iter() {
                write!(stdout, "{}", num.floor().to_int() as u8 as char)?;
            }
            stdout.flush()?;
            state.get_stack(1).clear();
        }

        if !state.get_stack(2).is_empty() {
            for num in state.get_stack(2).iter() {
                write!(stderr, "{}", num.floor().to_int() as u8 as char)?;
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
