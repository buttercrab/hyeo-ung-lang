use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};
use std::{env, process};

use colored::Colorize;

use crate::{code, parse};
use std::process::Command;

pub struct CustomWriter<T>
where
    T: Fn(String) -> std::io::Result<()>,
{
    buf: Vec<u8>,
    print_fn: T,
}

impl<T> Write for CustomWriter<T>
where
    T: Fn(String) -> std::io::Result<()>,
{
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.buf.append(&mut buf.to_vec());
        Result::Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        let res = (self.print_fn)(self.to_string());
        self.buf = Vec::new();
        res
    }
}

impl<T> CustomWriter<T>
where
    T: Fn(String) -> std::io::Result<()>,
{
    pub fn new(func: T) -> CustomWriter<T> {
        CustomWriter {
            buf: Vec::new(),
            print_fn: func,
        }
    }

    pub fn to_string(&self) -> String {
        handle_error(String::from_utf8(self.buf.clone()))
    }
}

pub trait ReadLine {
    fn read_line_(&mut self) -> String;
}

pub struct CustomReader {
    buf: Vec<String>,
    idx: usize,
}

impl ReadLine for std::io::Stdin {
    fn read_line_(&mut self) -> String {
        let mut res = String::new();
        handle_error(self.read_line(&mut res));
        res
    }
}

impl ReadLine for CustomReader {
    fn read_line_(&mut self) -> String {
        if self.buf.len() == self.idx {
            String::from("")
        } else {
            let res = self.buf[self.idx].clone();
            self.idx += 1;
            res
        }
    }
}

impl CustomReader {
    pub fn new(s: String) -> CustomReader {
        CustomReader {
            buf: s.split("\n").map(|x| String::from(x)).collect(),
            idx: 0,
        }
    }
}

pub fn read_file(file: &str) -> Vec<code::UnOptCode> {
    if !check_file(file) {
        print_error_string("only file with .hyeong supported");
    }
    print_log(&*format!("parsing {}", file));
    let res = parse::parse(handle_error(read_file_base(file)));
    print_log(&*format!("⮑  total {} commands", res.len()));
    res
}

fn read_file_base(file: &str) -> Result<String, std::io::Error> {
    let mut res = String::new();
    let mut f = File::open(file)?;

    f.read_to_string(&mut res)?;

    Ok(res)
}

fn check_file(file: &str) -> bool {
    file.rsplit(".").next() == Some("hyeong")
}

pub fn read_line() -> String {
    read_line_from(&mut std::io::stdin())
}

pub fn read_line_from(input: &mut impl ReadLine) -> String {
    input.read_line_()
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

pub fn print_error_string(err: &str) -> ! {
    println!("[{}] {}", "error".red(), err);
    process::exit(1);
}

pub fn print_error_no_exit(err: impl Error) {
    println!("[{}] {:?}", "error".red(), err);
}

pub fn print_error_str_no_exit(err: &str) {
    println!("[{}] {}", "error".red(), err);
}

pub fn print_log(msg: &str) {
    println!("{} {}", "====> ".blue(), msg);
}

pub fn print_warn(msg: &str) {
    println!("[{}] {}", "warn".yellow(), msg);
}

pub fn print_note(msg: &str) {
    println!("[{}] {}", "note".bright_cyan(), msg);
}

pub fn write<W>(w: &mut W, content: &str)
where
    W: Write,
{
    if let Err(e) = w.write_all(content.as_bytes()) {
        print_error(e);
    }
}

fn save_file_base(file: &str, content: String) -> Result<(), std::io::Error> {
    let mut file = File::create(file)?;
    file.write(content.as_bytes())?;
    Result::Ok(())
}

pub fn save_to_file(file: &str, content: String) {
    handle_error(save_file_base(file, content));
}

pub fn get_build_path() -> String {
    if cfg!(target_os = "windows") {
        env::var("USERPROFILE").unwrap() + "\\.hyeong\\hyeong-build"
    } else {
        env::var("HOME").unwrap() + "/.hyeong/hyeong-build"
    }
}

pub fn execute_command_stdout(windows: &str, linux: &str) {
    print!(
        "{}",
        handle_error(String::from_utf8(
            handle_error(if cfg!(target_os = "windows") {
                Command::new("cmd").arg("/C").arg(windows).output()
            } else {
                Command::new("bash").arg("-c").arg(linux).output()
            })
            .stdout
        ))
    );
}

pub fn execute_command_stderr(windows: &str, linux: &str) {
    print!(
        "{}",
        handle_error(String::from_utf8(
            handle_error(if cfg!(target_os = "windows") {
                Command::new("cmd").arg("/C").arg(windows).output()
            } else {
                Command::new("bash").arg("-c").arg(linux).output()
            })
            .stderr
        ))
    );
}
