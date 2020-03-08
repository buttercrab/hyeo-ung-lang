use crate::code::{State, UnOptCode};
use crate::{code, execute, io};
use colored::Colorize;
use std::collections::HashSet;
use std::io::{stdout, Write};
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

    let mut cur_loc = 0usize;
    let mut is_running = false;
    let mut break_points = HashSet::new();
    break_points.insert(from);

    let mut out = io::CustomWriter::new();
    let mut err = io::CustomWriter::new();

    while cur_loc < code.len() {
        let c = &code[cur_loc];
        if is_running {
            if break_points.contains(&cur_loc) {
                let out_str = out.to_string();
                let err_str = err.to_string();

                if !out_str.is_empty() {
                    println!("[{}] {}", "stdout".bold(), out_str);
                }

                if !err_str.is_empty() {
                    println!("[{}] {}", "stderr".bold().bright_red(), err_str);
                }

                out = io::CustomWriter::new();
                err = io::CustomWriter::new();

                is_running = false;
            } else {
                let (new_state, new_loc) = execute::execute_one(&mut out, &mut err, state, cur_loc);
                state = new_state;
                cur_loc = new_loc;
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

                if parsed.is_empty() {
                    continue;
                }

                match parsed[0] {
                    "next" | "n" => {
                        println!(
                            "{}:{}|{} {}",
                            c.get_location().0,
                            c.get_location().1,
                            cur_loc,
                            c.get_raw().bright_blue()
                        );

                        let (new_state, new_loc) =
                            execute::execute_one(&mut out, &mut err, state, cur_loc);
                        state = new_state;
                        cur_loc = new_loc;

                        let out_str = out.to_string();
                        let err_str = err.to_string();

                        if !out_str.is_empty() {
                            println!("[{}] {}", "stdout".bold(), out_str);
                        }

                        if !err_str.is_empty() {
                            println!("[{}] {}", "stderr".bold().bright_red(), err_str);
                        }

                        out = io::CustomWriter::new();
                        err = io::CustomWriter::new();

                        break;
                    }

                    "run" | "r" => {
                        let (new_state, new_loc) =
                            execute::execute_one(&mut out, &mut err, state, cur_loc);
                        state = new_state;
                        cur_loc = new_loc;

                        is_running = true;
                        break;
                    }

                    "state" | "s" => {
                        print!("{:?}", state);
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

                        break_points.insert(num);
                    }

                    "help" => {
                        println!("exit      Exit debugger");
                        println!("help      Print this");
                        println!("next(n)   goto next command");
                        println!("state(s)  print state status");
                        continue;
                    }

                    "exit" => {
                        process::exit(0);
                    }

                    t => {
                        println!("command \"{}\" not found", t);
                    }
                }
            }
        }
    }

    process::exit(0);
}
