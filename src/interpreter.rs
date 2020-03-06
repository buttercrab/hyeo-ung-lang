use std::io::{stdout, Write};
use std::process;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use colored::Colorize;

use crate::{code, execute, io, parse};

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
                io::print_error(e);
            }
            r.store(true, Ordering::SeqCst);
        }
    })
    .expect("Error setting Ctrl-C handler");

    println!("Hyeo-ung programming language");
    println!("type help for help!");

    loop {
        print!("{} ", ">".bright_blue());
        if let Result::Err(e) = stdout().flush() {
            io::print_error(e);
        }
        running.store(true, Ordering::SeqCst);
        let input = io::read_line();
        running.store(false, Ordering::SeqCst);

        if input == "" {
            process::exit(0);
        }

        let mut out = io::CustomWriter::new();
        let mut err = io::CustomWriter::new();

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
                    state = execute::execute(&mut out, &mut err, state, c);
                }
            }
        }

        let out_str = out.to_string();
        let err_str = err.to_string();

        if !out_str.is_empty() {
            println!("[{}] {}", "stdout".bold(), out_str);
        }

        if !err_str.is_empty() {
            println!("[{}] {}", "stderr".bold().bright_red(), err_str);
        }
    }
}
