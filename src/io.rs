use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};
use std::process;

use colored::Colorize;

use crate::{code, parse};

pub struct CustomWriter {
    buffer: Vec<u8>,
}

impl Write for CustomWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.buffer.append(&mut buf.to_vec());
        Result::Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Result::Ok(())
    }
}

impl CustomWriter {
    pub fn new() -> CustomWriter {
        CustomWriter { buffer: Vec::new() }
    }

    pub fn to_string(&self) -> String {
        match String::from_utf8(self.buffer.clone()) {
            Ok(value) => value,
            Err(e) => print_error(e),
        }
    }
}

pub fn read_file(file: &str) -> Vec<code::UnOptCode> {
    let code = match read_file_base(file) {
        Ok(t) => t,
        Err(e) => print_error(e),
    };

    parse::parse(code)
}

fn read_file_base(file: &str) -> Result<String, std::io::Error> {
    let mut res = String::new();
    let mut f = File::open(file)?;

    f.read_to_string(&mut res)?;

    Ok(res)
}

pub fn read_line() -> String {
    let mut res = String::new();
    match std::io::stdin().read_line(&mut res) {
        Ok(_) => res,
        Err(e) => print_error(e),
    }
}

pub fn handle_error<T>(res: Result<T, impl Error>) -> T {
    match res {
        Ok(value) => value,
        Err(e) => print_error(e),
    }
}

pub fn print_error(err: impl Error) -> ! {
    println!("[{}] {:?}", "error".red(), err);
    process::exit(1);
}

pub fn print_log(msg: &str) {
    println!("[{}] {}", "log".blue(), msg);
}

pub fn print_warn(msg: &str) {
    println!("[{}] {}", "warn".yellow(), msg);
}

pub fn print_note(msg: &str) {
    println!("[{}] {}", "note".bright_cyan(), msg);
}

pub fn write<W: Write>(w: &mut W, content: &str) {
    if let Err(e) = w.write_all(content.as_bytes()) {
        print_error(e);
    }
}
