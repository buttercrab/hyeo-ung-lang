use std::sync::atomic::{Ordering, AtomicBool};
use colored::Colorize;
use std::io::{stdout, Write};
use crate::{io, code, execute};
use std::sync::Arc;
use crate::code::UnOptCode;
use std::process;

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

    for c in code {
        loop {
            print!("{} ", ">".bright_red());
            io::handle_error(stdout().flush());
            running.store(true, Ordering::SeqCst);
            let input = io::read_line();
            running.store(false, Ordering::SeqCst);

            if input == "" {
                process::exit(0);
            }

            match input.trim() {
                "next" | "n" => {
                    println!("{}", c.get_raw().bright_blue());

                    let mut out = io::CustomWriter::new();
                    let mut err = io::CustomWriter::new();

                    state = execute::execute(&mut out, &mut err, state, &c);

                    let out_str = out.to_string();
                    let err_str = err.to_string();

                    if !out_str.is_empty() {
                        println!("[{}] {}", "stdout".bold(), out_str);
                    }

                    if !err_str.is_empty() {
                        println!("[{}] {}", "stderr".bold().bright_red(), err_str);
                    }

                    break;
                }

                "state" | "s" => {
                    print!("{:?}", state);
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

                "" => {
                    continue;
                }

                t => {
                    println!("command \"{}\" not found", t);
                }
            }
        }
    }

    process::exit(0);
}
