use crate::state::{State, UnOptState};
use crate::{execute, io, optimize, option};
use clap::{App, ArgMatches};
use std::io::Write;

#[cfg_attr(tarpaulin, skip)]
pub fn app<'a, 'b>() -> App<'a, 'b> {
    App::new("run")
        .about("Run hyeong code directly")
        .arg(option::input())
        .arg(option::optimize())
}

pub fn run(matches: &ArgMatches) {
    let file = matches.value_of("input").unwrap();
    let un_opt_code = io::read_file(file);
    let level_str = matches.value_of("optimize").unwrap();

    let level = io::handle_error(level_str.parse::<usize>());
    let mut stdout = std::io::stdout();
    let mut stderr = std::io::stderr();

    if level >= 1 {
        let (mut state, opt_code) = optimize::optimize(un_opt_code, level);
        io::print_log("running code");

        if !state.get_stack(1).is_empty() {
            for num in state.get_stack(1).iter() {
                io::write(
                    &mut stdout,
                    &*format!("{}", num.floor().to_int() as u8 as char),
                );
            }
            io::handle_error(stdout.flush());
            state.get_stack(1).clear();
        }

        if !state.get_stack(2).is_empty() {
            for num in state.get_stack(2).iter() {
                io::write(
                    &mut stderr,
                    &*format!("{}", num.floor().to_int() as u8 as char),
                );
            }
            io::handle_error(stderr.flush());
            state.get_stack(2).clear();
        }
        for code in opt_code {
            state = execute::execute(
                &mut std::io::stdin(),
                &mut stdout,
                &mut stderr,
                state,
                &code,
            );
        }
    } else {
        let mut state = UnOptState::new();
        io::print_log("running code");
        for code in un_opt_code {
            state = execute::execute(
                &mut std::io::stdin(),
                &mut stdout,
                &mut stderr,
                state,
                &code,
            );
        }
    }
}
