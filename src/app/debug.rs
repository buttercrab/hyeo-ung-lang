use crate::app::check;
use crate::core::execute;
use crate::core::state::{State, UnOptState};
use crate::util::error::Error;
use crate::util::option::HyeongOption;
use crate::util::{ext, io, option};
use clap::App;
use std::collections::HashSet;
use std::io::{stdin, Write};
use std::process;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use termcolor::{Color, ColorSpec, StandardStream, WriteColor};

/// App for debug
#[cfg(not(tarpaulin_include))]
pub fn app<'a>() -> App<'a> {
    App::new("debug")
        .about("Debug your code command by command")
        .arg(option::input())
}

/// Debug function
///
/// It works like interpreter but accepting commands below.
/// 1. [b] break       show breakpoints
/// 2. [b] break NUM   set/unset breakpoint on NUM
/// 3. exit            Exit debugger
/// 4. [h] help        Print this
/// 5. [n] next        goto next command
/// 6. [s] state       print state status
/// 7. [p] previous    move to previous state
/// 8. [r] run         run until breakpoint
#[cfg(not(tarpaulin_include))]
pub fn run(stdout: &mut StandardStream, hy_opt: &HyeongOption) -> Result<(), Error> {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    let color = hy_opt.color;
    let mut state = UnOptState::new();

    ctrlc::set_handler(move || {
        if r.load(Ordering::SeqCst) {
            r.store(false, Ordering::SeqCst);
            let mut stdout = StandardStream::stdout(color);
            write!(stdout, "\ntype \"exit\" to exit\n").unwrap();
            stdout
                .set_color(ColorSpec::new().set_fg(Some(Color::Red)).set_bold(true))
                .unwrap();
            write!(stdout, ">").unwrap();
            stdout.reset().unwrap();
            write!(stdout, " ").unwrap();
            stdout.flush().unwrap();
            r.store(true, Ordering::SeqCst);
        }
    })
    .expect("Error setting Ctrl-C handler");

    io::print_log(stdout, "running in debug mode")?;

    let un_opt_code = ext::parse_file(stdout, hy_opt.input.as_ref().unwrap(), hy_opt)?;

    for c in &un_opt_code {
        state.push_code(c.clone());
    }

    let mut is_running = false;
    let mut break_points = HashSet::new();
    break_points.insert(0);

    let mut out = io::CustomWriter::new(|x| {
        if !x.is_empty() {
            let mut stdout = StandardStream::stdout(hy_opt.color);
            write!(stdout, "[")?;
            stdout.set_color(ColorSpec::new().set_bold(true))?;
            write!(stdout, "stdout")?;
            stdout.reset()?;
            writeln!(stdout, "] {}", x)?;
            stdout.flush()?;
        }

        Ok(())
    });

    let mut err = io::CustomWriter::new(|x| {
        if !x.is_empty() {
            let mut stdout = StandardStream::stdout(hy_opt.color);
            write!(stdout, "[")?;
            stdout.set_color(ColorSpec::new().set_bold(true).set_fg(Some(Color::Red)))?;
            write!(stdout, "stderr")?;
            stdout.reset()?;
            writeln!(stdout, "] {}", x)?;
            stdout.flush()?;
        }

        Ok(())
    });

    let mut state_stack = vec![(state, 0)];

    while state_stack.last().unwrap().1 < un_opt_code.len() {
        if is_running {
            if break_points.contains(&state_stack.last().unwrap().1) {
                out.flush().unwrap();
                err.flush().unwrap();
                is_running = false;
            } else {
                state_stack.push(execute::execute_one(
                    &mut stdin(),
                    &mut out,
                    &mut err,
                    state_stack.last().unwrap().0.clone(),
                    state_stack.last().unwrap().1,
                )?);
            }
        } else {
            loop {
                stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)).set_bold(true))?;
                write!(stdout, ">")?;
                stdout.reset()?;
                write!(stdout, " ")?;
                stdout.flush()?;
                running.store(true, Ordering::SeqCst);
                let input = io::read_line_from(&mut std::io::stdin())?;
                running.store(false, Ordering::SeqCst);

                if input == *"" {
                    process::exit(0);
                }

                let parsed = input.trim().split(' ').collect::<Vec<_>>();

                match parsed[0] {
                    "next" | "n" => {
                        let c = &un_opt_code[state_stack.last().unwrap().1];

                        check::print_un_opt_codes(
                            stdout,
                            hy_opt,
                            vec![(state_stack.last().unwrap().1, c)],
                            Color::Cyan,
                            true,
                        )?;

                        state_stack.push(execute::execute_one(
                            &mut stdin(),
                            &mut out,
                            &mut err,
                            state_stack.last().unwrap().0.clone(),
                            state_stack.last().unwrap().1,
                        )?);

                        out.flush().unwrap();
                        err.flush().unwrap();

                        break;
                    }

                    "previous" | "p" => {
                        if state_stack.len() > 1 {
                            state_stack.pop();
                            io::print_log(stdout, "moved back")?;
                        } else {
                            io::print_error_str_no_exit(stdout, "can't go back");
                        }
                    }

                    "run" | "r" => {
                        state_stack.push(execute::execute_one(
                            &mut stdin(),
                            &mut out,
                            &mut err,
                            state_stack.last().unwrap().0.clone(),
                            state_stack.last().unwrap().1,
                        )?);

                        is_running = true;
                        break;
                    }

                    "state" | "s" => {
                        write!(stdout, "{:?}", state_stack.last().unwrap().0)?;
                    }

                    "break" | "b" => {
                        if parsed.len() < 2 {
                            io::print_log(stdout, "printing breakpoints")?;
                            let mut v = break_points.iter().collect::<Vec<_>>();
                            v.sort();
                            check::print_un_opt_codes(
                                stdout,
                                hy_opt,
                                v.iter()
                                    .map(|&i| (*i, &un_opt_code[*i]))
                                    .collect::<Vec<_>>(),
                                Color::Red,
                                true,
                            )?;
                            continue;
                        }
                        let num = match parsed[1].parse::<usize>() {
                            Ok(t) => t,
                            Err(e) => {
                                io::print_error_no_exit(stdout, Error::from(e, ""));
                                continue;
                            }
                        };
                        if num > un_opt_code.len() {
                            io::print_error_str_no_exit(stdout, "number exceeds the range");
                            continue;
                        }

                        if !break_points.contains(&num) {
                            break_points.insert(num);
                            io::print_log(stdout, format!("set breakpoint on line {}", num))?;
                        } else {
                            break_points.remove(&num);
                            io::print_log(stdout, format!("unset breakpoint on line {}", num))?;
                        }
                    }

                    "help" | "h" => {
                        writeln!(stdout, "[b] break       show breakpoints")?;
                        writeln!(stdout, "[b] break NUM   set/unset breakpoint on NUM")?;
                        writeln!(stdout, "exit            Exit debugger")?;
                        writeln!(stdout, "[h] help        Print this")?;
                        writeln!(stdout, "[n] next        goto next command")?;
                        writeln!(stdout, "[s] state       print state status")?;
                        writeln!(stdout, "[p] previous    move to previous state")?;
                        writeln!(stdout, "[r] run         run until breakpoint")?;
                        continue;
                    }

                    "exit" => {
                        process::exit(0);
                    }

                    "" => {
                        continue;
                    }

                    t => {
                        io::print_error_str_no_exit(stdout, format!("command \"{}\" not found", t));
                    }
                }
            }
        }
    }

    out.flush().unwrap();
    err.flush().unwrap();

    Ok(())
}
