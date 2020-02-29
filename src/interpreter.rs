use std::io::{stdout, Write};
use std::process;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use colored::Colorize;

use crate::{execute, io, parse};

pub struct Interpreter {
    state: execute::State
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            state: execute::State::new()
        }
    }

    pub fn run(&mut self) -> ! {
        let running = Arc::new(AtomicBool::new(true));
        let r = running.clone();

        ctrlc::set_handler(move || {
            if r.load(Ordering::SeqCst) {
                r.store(false, Ordering::SeqCst);
                print!("\ntype \"흑.하앙...\" or \"exit\" to exit\n> ");
                stdout().flush();
                r.store(true, Ordering::SeqCst);
            }
        }).expect("Error setting Ctrl-C handler");

        println!("Hyeo-ung programming language");
        println!("type help for help");

        loop {
            print!("{} ", ">".bright_blue());
            stdout().flush();
            running.store(true, Ordering::SeqCst);
            let input = io::read_line();
            running.store(false, Ordering::SeqCst);

            match input.as_str() {
                "\n" => {
                    continue;
                }

                "help\n" => {
                    println!("help  Print this");
                    println!("exit  Exit this interpreter");
                    println!("      You can also exit by typing \"흑.하앙...\"");
                    continue;
                }

                "" | "exit\n" => {
                    process::exit(0);
                }

                _ => {
                    let code = parse::parse(input);
                    for c in code.iter() {
                        execute::execute(&mut self.state, c);
                    }
                }
            }
        }
    }
}