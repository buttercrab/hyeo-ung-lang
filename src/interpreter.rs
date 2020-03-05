use std::io::{stderr, stdout, Write};
use std::process;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use colored::Colorize;

use crate::{code, execute, io, parse};
use crate::io::print_error;

#[cfg_attr(tarpaulin, skip)]
pub fn run() -> ! {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    let mut state = code::UnOptState::new();

    ctrlc::set_handler(move || {
        if r.load(Ordering::SeqCst) {
            r.store(false, Ordering::SeqCst);
            print!("\ntype \"흑.하앙...\" or \"exit\" to exit\n");
            print!("{} ", ">".bright_blue());
            if let Result::Err(e) = stdout().flush() {
                print_error(e);
            }
            r.store(true, Ordering::SeqCst);
        }
    }).expect("Error setting Ctrl-C handler");

    println!("Hyeo-ung programming language");
    println!("type help for help!");

    loop {
        print!("{} ", ">".bright_blue());
        if let Result::Err(e) = stdout().flush() {
            print_error(e);
        }
        running.store(true, Ordering::SeqCst);
        let input = io::read_line();
        running.store(false, Ordering::SeqCst);

        if input == "" {
            process::exit(0);
        }

        match input.trim() {
            "" => {
                continue;
            }

            "help" => {
                println!("help  Print this");
                println!("exit  Exit this interpreter");
                println!("      You can also exit by typing \"흑.하앙...\"");
                continue;
            }

            "exit" => {
                process::exit(0);
            }

            _ => {
                let code = parse::parse(input);
                for c in code.iter() {
                    state = execute::execute(&mut stdout(), &mut stderr(), state, c);
                }
            }
        }
    }
}
