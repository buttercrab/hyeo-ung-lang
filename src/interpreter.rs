use std::io::{stdout, Write};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

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
        let mut state = self.state.clone();
        let running = Arc::new(AtomicBool::new(true));
        let r = running.clone();
        ctrlc::set_handler(move || {
            if r.load(Ordering::SeqCst) {
                r.store(false, Ordering::SeqCst);
                print!("\ntype \"흑.하앙...\" to exit\n> ");
                stdout().flush();
                r.store(true, Ordering::SeqCst);
            }
        }).expect("Error setting Ctrl-C handler");
        loop {
            print!("> ");
            stdout().flush();
            running.store(true, Ordering::SeqCst);
            let input = io::read_line();
            running.store(false, Ordering::SeqCst);
            let code = match parse::parse(input) {
                Ok(t) => t,
                Err(e) => io::print_error(e),
            };
            for c in code.iter() {
                state = execute::execute(state, c);
            }
        }
    }
}