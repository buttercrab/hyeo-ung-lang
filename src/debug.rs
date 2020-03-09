use crate::code::{State, UnOptCode};
use crate::{code, execute, io};
use colored::Colorize;
use std::collections::HashSet;
use std::io::{stdin, stdout, Write};
use std::process;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub fn run(code: Vec<UnOptCode>, from: usize) -> ! {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    let mut state = code::UnOptState::new();

    ctrlc::set_handler(move || {
        if r.load(Ordering::SeqCst) {
            r.store(false, Ordering::SeqCst);
            print!("\ntype \"exit\" to exit\n");
            print!("{} ", ">".bright_red());
            io::handle_error(stdout().flush());
            r.store(true, Ordering::SeqCst);
        }
    })
    .expect("Error setting Ctrl-C handler");

    io::print_log("running in debug mode");

    for c in &code {
        state.push_code(c.clone());
    }

    let mut is_running = false;
    let mut break_points = HashSet::new();
    break_points.insert(from);

    let mut out = io::CustomWriter::new(|x| {
        if !x.is_empty() {
            println!("[{}] {}", "stdout".bold(), x);
        }

        Result::Ok(())
    });

    let mut err = io::CustomWriter::new(|x| {
        if !x.is_empty() {
            println!("[{}] {}", "stderr".bold().bright_red(), x);
        }

        Result::Ok(())
    });

    let mut state_stack = vec![(state, 0)];

    while state_stack.last().unwrap().1 < code.len() {
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
                ));
            }
        } else {
            loop {
                print!("{} ", ">".bright_red());
                io::handle_error(stdout().flush());
                running.store(true, Ordering::SeqCst);
                let input = io::read_line();
                running.store(false, Ordering::SeqCst);

                if input == "" {
                    process::exit(0);
                }

                let parsed = input.trim().split(" ").collect::<Vec<_>>();

                match parsed[0] {
                    "next" | "n" => {
                        let c = &code[state_stack.last().unwrap().1];

                        println!(
                            "{}:{}|{} {}",
                            c.get_location().0,
                            c.get_location().1,
                            state_stack.last().unwrap().1,
                            c.get_raw().bright_blue()
                        );

                        state_stack.push(execute::execute_one(
                            &mut stdin(),
                            &mut out,
                            &mut err,
                            state_stack.last().unwrap().0.clone(),
                            state_stack.last().unwrap().1,
                        ));

                        out.flush().unwrap();
                        err.flush().unwrap();

                        break;
                    }

                    "previous" | "p" => {
                        if state_stack.len() > 1 {
                            state_stack.pop();
                            io::print_log("moved back");
                        } else {
                            io::print_error_str_no_exit("cannot go back");
                        }
                    }

                    "run" | "r" => {
                        state_stack.push(execute::execute_one(
                            &mut stdin(),
                            &mut out,
                            &mut err,
                            state_stack.last().unwrap().0.clone(),
                            state_stack.last().unwrap().1,
                        ));

                        is_running = true;
                        break;
                    }

                    "state" | "s" => {
                        print!("{:?}", state_stack.last().unwrap().0);
                    }

                    "break" | "b" => {
                        if parsed.len() < 2 {
                            let mut v = break_points.iter().collect::<Vec<_>>();
                            v.sort();
                            for i in v {
                                println!("{}: {}", i, code[*i].get_raw());
                            }
                            continue;
                        }
                        let num = match parsed[1].parse::<usize>() {
                            Ok(t) => t,
                            Err(e) => {
                                io::print_error_no_exit(e);
                                continue;
                            }
                        };
                        if num > code.len() {
                            io::print_error_str_no_exit("number exceeds the range");
                            continue;
                        }

                        if !break_points.contains(&num) {
                            break_points.insert(num);
                            io::print_log(&*format!("set breakpoint on line {}", num));
                        } else {
                            break_points.remove(&num);
                            io::print_log(&*format!("unset breakpoint on line {}", num));
                        }
                    }

                    "help" | "h" => {
                        println!("break(b)       show breakpoints");
                        println!("break(b) NUM   set/unset breakpoint on NUM");
                        println!("exit           Exit debugger");
                        println!("help(h)        Print this");
                        println!("next(n)        goto next command");
                        println!("state(s)       print state status");
                        println!("previous(p)    move to previous state");
                        println!("run(r)         run until breakpoint");
                        continue;
                    }

                    "exit" => {
                        process::exit(0);
                    }

                    "" => {
                        continue;
                    }

                    t => {
                        println!("command \"{}\" not found", t);
                    }
                }
            }
        }
    }

    out.flush().unwrap();
    err.flush().unwrap();

    process::exit(0);
}
