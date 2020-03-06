use std::io::{stdout, Write};
use std::process;
use std::string::FromUtf8Error;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use colored::Colorize;

use crate::{code, execute, io, parse};

#[cfg_attr(tarpaulin, skip)]
struct CustomWriter {
    buffer: Vec<u8>,
}

#[cfg_attr(tarpaulin, skip)]
impl Write for CustomWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.buffer.append(&mut buf.to_vec());
        Result::Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Result::Ok(())
    }
}

#[cfg_attr(tarpaulin, skip)]
impl CustomWriter {
    fn new() -> CustomWriter {
        CustomWriter {
            buffer: Vec::new(),
        }
    }

    fn to_string(&self) -> String {
        match String::from_utf8(self.buffer.clone()) {
            Ok(value) => value,
            Err(e) => io::print_error(e),
        }
    }
}

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
    }).expect("Error setting Ctrl-C handler");

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

        let mut out = CustomWriter::new();
        let mut err = CustomWriter::new();

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
