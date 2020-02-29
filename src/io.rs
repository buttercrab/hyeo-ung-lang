use std::{io, process};
use std::error::Error;
use std::fs::File;
use std::io::Read;

use colored::Colorize;

use crate::parse;

pub fn read_file(file: &str) -> Vec<parse::Command> {
    let code = match read_file_base(file) {
        Ok(t) => t,
        Err(e) => print_error(e),
    };

    parse::parse(code)
}

fn read_file_base(file: &str) -> Result<String, io::Error> {
    let mut res = String::new();
    let mut f = File::open(file)?;

    f.read_to_string(&mut res)?;

    Ok(res)
}

pub fn print_error<T: Error>(err: T) -> ! {
    println!("[{}] {:?}", "error".red(), err);
    process::exit(1);
}

pub fn read_line() -> String {
    let mut res = String::new();
    match io::stdin().read_line(&mut res) {
        Ok(t) => res,
        Err(e) => print_error(e),
    }
}