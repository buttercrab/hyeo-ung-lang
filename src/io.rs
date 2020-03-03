use std::{io, process};
use std::error::Error;
use std::fs::File;
use std::io::Read;

use colored::Colorize;

use crate::{code, parse};

#[cfg_attr(tarpaulin, skip)]
pub fn read_file(file: &str) -> Vec<code::UnOptCode> {
    let code = match read_file_base(file) {
        Ok(t) => t,
        Err(e) => print_error(e),
    };

    parse::parse(code)
}

#[cfg_attr(tarpaulin, skip)]
fn read_file_base(file: &str) -> Result<String, io::Error> {
    let mut res = String::new();
    let mut f = File::open(file)?;

    f.read_to_string(&mut res)?;

    Ok(res)
}

#[cfg_attr(tarpaulin, skip)]
pub fn read_line() -> String {
    let mut res = String::new();
    match io::stdin().read_line(&mut res) {
        Ok(_) => res,
        Err(e) => print_error(e),
    }
}

#[cfg_attr(tarpaulin, skip)]
pub fn print_error<T: Error>(err: T) -> ! {
    println!("[{}] {:?}", "error".red(), err);
    process::exit(1);
}

#[cfg_attr(tarpaulin, skip)]
pub fn print_log(msg: &str) {
    println!("[{}] {}", "log".blue(), msg);
}

#[cfg_attr(tarpaulin, skip)]
pub fn print_warn(msg: &str) {
    println!("[{}] {}", "warn".yellow(), msg);
}

#[cfg_attr(tarpaulin, skip)]
pub fn print_note(msg: &str) {
    println!("[{}] {}", "note".bright_cyan(), msg);
}