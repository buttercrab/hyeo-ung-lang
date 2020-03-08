use std::io::{stdout, Write};
use std::process;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use colored::Colorize;

use crate::{code, execute, io, parse};

pub fn run() -> ! {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    let mut state = code::UnOptState::new();

    ctrlc::set_handler(move || {
        if r.load(Ordering::SeqCst) {
            r.store(false, Ordering::SeqCst);
            print!("\ntype \"흑.하앙...\" or \"exit\" to exit\n");
            print!("{} ", ">".bright_blue());
            io::handle_error(stdout().flush());
            r.store(true, Ordering::SeqCst);
        }
    })
    .expect("Error setting Ctrl-C handler");

    println!("Hyeo-ung Programming Language");
    println!("type help for help");

    loop {
        print!("{} ", ">".bright_blue());
        io::handle_error(stdout().flush());
        running.store(true, Ordering::SeqCst);
        let input = io::read_line();
        running.store(false, Ordering::SeqCst);

        if input == "" {
            process::exit(0);
        }

        let mut out = io::CustomWriter::new(|x| {
            let out_str = io::handle_error(String::from_utf8(x.clone()));

            if !out_str.is_empty() {
                println!("[{}] {}", "stdout".bold(), out_str);
            }

            Result::Ok(())
        });

        let mut err = io::CustomWriter::new(|x| {
            let err_str = io::handle_error(String::from_utf8(x.clone()));

            if !err_str.is_empty() {
                println!("[{}] {}", "stderr".bold().bright_red(), err_str);
            }

            Result::Ok(())
        });

        match input.trim() {
            "" => {
                continue;
            }

            "clear" => {
                state = code::UnOptState::new();
            }

            "help" => {
                println!("clear  Clears the state");
                println!("exit   Exit this interpreter");
                println!("       You can also exit by typing \"흑.하앙...\"");
                println!("help   Print this");
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

        out.flush().unwrap();
        err.flush().unwrap();
    }
}
