use std::{io, process};
use std::error::Error;
use std::fs::File;
use std::io::Read;

use colored::Colorize;

use crate::parse::Command;

pub struct IO;

impl IO {
    pub fn read_file(file: &str) -> Vec<Command> {
        let code = match IO::read_file_base(file) {
            Ok(t) => t,
            Err(e) => IO::print_error(e),
        };

        match Command::parse(code) {
            Ok(t) => t,
            Err(e) => IO::print_error(e),
        }
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
}