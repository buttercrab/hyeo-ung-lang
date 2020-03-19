use crate::code::UnOptCode;
use crate::parse;
use colored::Colorize;
use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};
use std::process::Command;
use std::{env, process};

/// Custom writer structure for redirecting output.
///
/// # Examples
///
/// ```
/// use hyeong::io::CustomWriter;
/// use std::io::Write;
///
/// let mut  a = CustomWriter::new(|_| Result::Ok(()));
///
/// a.write_all("Hello, World!".as_bytes()).unwrap();
/// assert_eq!("Hello, World!", a.to_string());
/// ```
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
    /// Make new `CustomWriter`
    ///
    /// # Examples
    ///
    /// ```
    /// use hyeong::io::CustomWriter;
    /// use std::io::Write;
    ///
    /// let mut  a = CustomWriter::new(|_| Result::Ok(()));
    ///
    /// a.write_all("Hello, World!".as_bytes()).unwrap();
    /// assert_eq!("Hello, World!", a.to_string());
    /// ```
    pub fn new(func: T) -> CustomWriter<T> {
        CustomWriter {
            buf: Vec::new(),
            print_fn: func,
        }
    }

    /// Return string that is written
    pub fn to_string(&self) -> String {
        handle_error(String::from_utf8(self.buf.clone()))
    }
}

/// ReadLine trait that is used in reading lines
/// This is made for uniting `std::io::Stdin` and `CustomReader`
pub trait ReadLine {
    fn read_line_(&mut self) -> String;
}

/// Custom reader structure for other input source
///
/// # Examples
///
/// ```
/// use hyeong::io::{CustomReader, ReadLine};
///
/// let mut a = CustomReader::new("Hello, World!".to_string());
///
/// assert_eq!("Hello, World!", a.read_line_());
/// ```
pub struct CustomReader {
    buf: Vec<String>,
    idx: usize,
}

impl ReadLine for std::io::Stdin {
    /// `read_line` wrapper
    #[cfg_attr(tarpaulin, skip)]
    fn read_line_(&mut self) -> String {
        let mut res = String::new();
        handle_error(self.read_line(&mut res));
        res
    }
}

impl ReadLine for CustomReader {
    /// Reads until next line character(`\n`)
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
    /// Make new `CustomReader`
    ///
    /// # Examples
    ///
    /// ```
    /// use hyeong::io::{CustomReader, ReadLine};
    ///
    /// let mut a = CustomReader::new("Hello, World!".to_string());
    ///
    /// assert_eq!("Hello, World!", a.read_line_());
    /// ```
    pub fn new(s: String) -> CustomReader {
        CustomReader {
            buf: s.split("\n").map(|x| String::from(x)).collect(),
            idx: 0,
        }
    }
}

/// Read `.hyeong` file and parse to code
pub fn read_file(file: &str) -> Vec<UnOptCode> {
    if !check_file(file) {
        print_error_string("only file with .hyeong supported");
    }
    print_log(&*format!("parsing {}", file));
    let res = parse::parse(handle_error(read_file_base(file)));
    print_log(&*format!("⮑  total {} commands", res.len()));
    res
}

/// Base function of `read_file`
/// It reads any file and return to string
fn read_file_base(file: &str) -> Result<String, std::io::Error> {
    let mut res = String::new();
    let mut f = File::open(file)?;

    f.read_to_string(&mut res)?;

    Ok(res)
}

/// Check if file extension is `.hyeong`
fn check_file(file: &str) -> bool {
    file.rsplit(".").next() == Some("hyeong")
}

/// Read line from stdin
#[cfg_attr(tarpaulin, skip)]
pub fn read_line() -> String {
    read_line_from(&mut std::io::stdin())
}

/// Read line from `ReadLine`
pub fn read_line_from(input: &mut impl ReadLine) -> String {
    input.read_line_()
}

/// If `res` is Err, it prints error and exit
/// If is not, is would unwrap
///
/// # Examples
///
/// ```
/// use hyeong::io;
/// use std::io::Error;
///
/// let a = Result::<i32, Error>::Ok(1);
///
/// assert_eq!(1, io::handle_error(a));
/// ```
pub fn handle_error<T>(res: Result<T, impl Error>) -> T {
    match res {
        Ok(value) => value,
        Err(e) => print_error(e),
    }
}

/// Print error and terminate
#[cfg_attr(tarpaulin, skip)]
pub fn print_error(err: impl Error) -> ! {
    println!("[{}] {:?}", "error".red(), err);
    process::exit(1);
}

/// Print error string and terminate
#[cfg_attr(tarpaulin, skip)]
pub fn print_error_string(err: &str) -> ! {
    println!("[{}] {}", "error".red(), err);
    process::exit(1);
}

/// Print error
#[cfg_attr(tarpaulin, skip)]
pub fn print_error_no_exit(err: impl Error) {
    println!("[{}] {:?}", "error".red(), err);
}

/// Print error string
#[cfg_attr(tarpaulin, skip)]
pub fn print_error_str_no_exit(err: &str) {
    println!("[{}] {}", "error".red(), err);
}

/// Print log
#[cfg_attr(tarpaulin, skip)]
pub fn print_log(msg: &str) {
    println!("{} {}", "==>".blue(), msg.bold());
}

/// Print warning
#[cfg_attr(tarpaulin, skip)]
pub fn print_warn(msg: &str) {
    println!("[{}] {}", "warn".yellow(), msg);
}

/// Print note
#[cfg_attr(tarpaulin, skip)]
pub fn print_note(msg: &str) {
    println!("[{}] {}", "note".bright_cyan(), msg);
}

/// Write to `Write`
///
/// # Examples
///
/// ```
/// use hyeong::io::CustomWriter;
/// use hyeong::io;
///
/// let mut a = CustomWriter::new(|_| Result::Ok(()));
/// io::write(&mut a, "Hello, World!");
///
/// assert_eq!("Hello, World!", a.to_string());
/// ```
pub fn write<W>(w: &mut W, content: &str)
where
    W: Write,
{
    if let Err(e) = w.write_all(content.as_bytes()) {
        print_error(e);
    }
}

/// Base function for saving to file
fn save_file_base(file: &str, content: String) -> Result<(), std::io::Error> {
    let mut file = File::create(file)?;
    file.write(content.as_bytes())?;
    Result::Ok(())
}

/// Save content to file
pub fn save_to_file(file: &str, content: String) {
    handle_error(save_file_base(file, content));
}

/// Get build path
/// Differ from os
#[cfg_attr(tarpaulin, skip)]
pub fn get_build_path() -> String {
    if cfg!(target_os = "windows") {
        env::var("USERPROFILE").unwrap() + "\\.hyeong\\hyeong-build"
    } else {
        env::var("HOME").unwrap() + "/.hyeong/hyeong-build"
    }
}

/// Execute command and prints stdout of command output
#[cfg_attr(tarpaulin, skip)]
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

/// Execute command and prints stderr of command output
#[cfg_attr(tarpaulin, skip)]
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

/// Execute command only
#[cfg_attr(tarpaulin, skip)]
pub fn execute_command(windows: &str, linux: &str) {
    if cfg!(target_os = "windows") {
        Command::new("cmd").arg("/C").arg(windows).output().unwrap();
    } else {
        Command::new("bash").arg("-c").arg(linux).output().unwrap();
    }
}
