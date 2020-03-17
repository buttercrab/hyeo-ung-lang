use crate::state::UnOptState;
use crate::{execute, io, parse};
use colored::Colorize;
use std::io::{stdin, stdout, Write};
use std::process;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

#[cfg_attr(tarpaulin, skip)]
pub fn run() -> ! {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    let mut state = UnOptState::new();

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

        match input.trim() {
            "" => {
                continue;
            }

            "clear" => {
                state = UnOptState::new();
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
                    state = execute::execute(&mut stdin(), &mut out, &mut err, state, c);
                }
            }
        }

        out.flush().unwrap();
        err.flush().unwrap();
    }
}
