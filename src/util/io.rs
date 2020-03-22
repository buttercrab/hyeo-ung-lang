use crate::core::code::UnOptCode;
use crate::core::parse;
use crate::util::error::Error;
use crate::util::util;
use std::ffi::OsStr;
use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind, Read, Write};
use std::path::PathBuf;
use std::process;
use std::process::{Command, Stdio};
use termcolor::{Color, ColorSpec, StandardStream, WriteColor};

/// Custom writer structure for redirecting output.
///
/// # Examples
///
/// ```
/// use std::io::Write;
/// use hyeong::util::io::CustomWriter;
///
/// let mut  a = CustomWriter::new(|_| Result::Ok(()));
///
/// a.write_all("Hello, World!".as_bytes()).unwrap();
/// assert_eq!("Hello, World!", a.to_string().unwrap());
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
        let res = (self.print_fn)(self.to_string()?);
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
    /// use std::io::Write;
    /// use hyeong::util::io::CustomWriter;
    ///
    /// let mut  a = CustomWriter::new(|_| Result::Ok(()));
    ///
    /// a.write_all("Hello, World!".as_bytes()).unwrap();
    /// assert_eq!("Hello, World!", a.to_string().unwrap());
    /// ```
    pub fn new(func: T) -> CustomWriter<T> {
        CustomWriter {
            buf: Vec::new(),
            print_fn: func,
        }
    }

    /// Return string that is written
    pub fn to_string(&self) -> Result<String, Error> {
        Ok(String::from_utf8(self.buf.clone())?)
    }
}

/// ReadLine trait that is used in reading lines
/// This is made for uniting `std::io::Stdin` and `CustomReader`
pub trait ReadLine {
    fn read_line_(&mut self) -> Result<String, Error>;
}

/// Custom reader structure for other input source
///
/// # Examples
///
/// ```
/// use hyeong::util::io::{CustomReader, ReadLine};
///
/// let mut a = CustomReader::new("Hello, World!".to_string());
///
/// assert_eq!("Hello, World!", a.read_line_().unwrap());
/// ```
pub struct CustomReader {
    buf: Vec<String>,
    idx: usize,
}

impl ReadLine for std::io::Stdin {
    /// `read_line` wrapper
    #[cfg_attr(tarpaulin, skip)]
    fn read_line_(&mut self) -> Result<String, Error> {
        let mut res = String::new();
        self.read_line(&mut res)?;
        Ok(res)
    }
}

impl ReadLine for CustomReader {
    /// Reads until next line character(`\n`)
    fn read_line_(&mut self) -> Result<String, Error> {
        if self.buf.len() == self.idx {
            Ok(String::from(""))
        } else {
            let res = self.buf[self.idx].clone();
            self.idx += 1;
            Ok(res)
        }
    }
}

impl CustomReader {
    /// Make new `CustomReader`
    ///
    /// # Examples
    ///
    /// ```
    /// use hyeong::util::io::{CustomReader, ReadLine};
    ///
    /// let mut a = CustomReader::new("Hello, World!".to_string());
    ///
    /// assert_eq!("Hello, World!", a.read_line_().unwrap());
    /// ```
    pub fn new(s: String) -> CustomReader {
        CustomReader {
            buf: s.split("\n").map(|x| String::from(x)).collect(),
            idx: 0,
        }
    }
}

/// Read .hyeong file
pub fn read_file(path: &PathBuf) -> Result<String, Error> {
    match path.extension() {
        Some(p) => {
            if p == OsStr::new("hyeong") {
                let mut buf = String::new();
                let mut f = File::open(path)?;
                f.read_to_string(&mut buf)?;
                Ok(buf)
            } else {
                Err(std::io::Error::new(
                    ErrorKind::InvalidInput,
                    "Only .hyeong extension supported",
                ))?
            }
        }
        _ => Err(std::io::Error::new(
            ErrorKind::InvalidInput,
            "Only .hyeong extension supported",
        ))?,
    }
}

pub fn parse_file(stdout: &mut StandardStream, path: &PathBuf) -> Result<Vec<UnOptCode>, Error> {
    let raw_code = read_file(path)?;
    print_log(stdout, format!("parsing {}", util::path_to_string(path)?))?;
    let un_opt_code = parse::parse(raw_code);
    print_log(stdout, format!("⮑ total {} commands", un_opt_code.len()))?;
    Ok(un_opt_code)
}

/// Read line from stdin
#[cfg_attr(tarpaulin, skip)]
pub fn read_line() -> Result<String, Error> {
    read_line_from(&mut std::io::stdin())
}

/// Read line from `ReadLine`
pub fn read_line_from(input: &mut impl ReadLine) -> Result<String, Error> {
    input.read_line_()
}

/// If `res` is Err, it prints error and exit
/// If is not, is would unwrap
///
/// # Examples
///
/// ```
/// use termcolor::{StandardStream, ColorChoice};
/// use hyeong::util::io;
/// use hyeong::util::error::Error;
///
/// let a = Result::<i32, Error>::Ok(1);
///
/// assert_eq!(1, io::handle(&mut StandardStream::stderr(ColorChoice::Never), a));
/// ```
pub fn handle<T>(w: &mut StandardStream, res: Result<T, Error>) -> T {
    match res {
        Ok(value) => value,
        Err(e) => print_error(w, e),
    }
}

/// Print error and terminate
#[cfg_attr(tarpaulin, skip)]
pub fn print_error(w: &mut StandardStream, err: Error) -> ! {
    print_error_no_exit(w, err);
    process::exit(1);
}

/// Print error string and terminate
#[cfg_attr(tarpaulin, skip)]
pub fn print_error_str<S>(w: &mut StandardStream, err: S) -> !
where
    S: Display,
{
    print_error_str_no_exit(w, err);
    process::exit(1);
}

/// Print error
#[cfg_attr(tarpaulin, skip)]
pub fn print_error_no_exit(w: &mut StandardStream, err: Error) {
    print_error_str_no_exit(w, format!("{:?}", err));
}

/// Print error string
#[cfg_attr(tarpaulin, skip)]
pub fn print_error_str_no_exit<S>(w: &mut StandardStream, err: S)
where
    S: Display,
{
    write!(w, "[").unwrap();
    w.set_color(ColorSpec::new().set_fg(Some(Color::Red)))
        .unwrap();
    write!(w, "error").unwrap();
    w.reset().unwrap();
    write!(w, "] ").unwrap();
    w.set_color(ColorSpec::new().set_bold(true)).unwrap();
    write!(w, "{}", err).unwrap();
    w.reset().unwrap();
    write!(w, "\n").unwrap();
}

/// Print log
#[cfg_attr(tarpaulin, skip)]
pub fn print_log<S>(w: &mut StandardStream, msg: S) -> Result<(), Error>
where
    S: Display,
{
    w.set_color(ColorSpec::new().set_fg(Some(Color::Blue)))?;
    write!(w, "==>")?;
    w.reset()?;
    write!(w, " ")?;
    w.set_color(ColorSpec::new().set_bold(true))?;
    write!(w, "{}", msg)?;
    w.reset()?;
    write!(w, "\n")?;
    Ok(())
}

/// Print note
#[cfg_attr(tarpaulin, skip)]
pub fn print_note<S>(w: &mut StandardStream, msg: S) -> Result<(), Error>
where
    S: Display,
{
    write!(w, "[")?;
    w.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)))?;
    write!(w, "note")?;
    w.reset()?;
    write!(w, "] ")?;
    w.set_color(ColorSpec::new().set_bold(true))?;
    write!(w, "{}", msg)?;
    w.reset()?;
    write!(w, "\n")?;
    Ok(())
}

/// Save content to file
pub fn save_to_file(path: &PathBuf, content: String) -> Result<(), Error> {
    let mut file = File::create(path)?;
    file.write(content.as_bytes())?;
    Ok(())
}

/// Execute command only
#[cfg_attr(tarpaulin, skip)]
pub fn execute_command_stdout(w: &mut StandardStream, command: &str) -> Result<(), Error> {
    let mut cmd = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .arg("/C")
            .arg(command)
            .stdout(Stdio::piped())
            .spawn()?
    } else {
        Command::new("bash")
            .arg("-c")
            .arg(command)
            .stdout(Stdio::piped())
            .spawn()?
    };

    let stdout = cmd.stdout.as_mut().unwrap();
    let stdout_reader = BufReader::new(stdout);

    for line in stdout_reader.lines() {
        write!(w, "{}\n", line?)?;
    }

    let e = cmd.wait()?;

    if e.success() {
        Ok(())
    } else {
        match e.code() {
            Some(code) => Err(Error::new(
                format!("command {} failed with exit code {}", command, code),
                String::from(""),
            )),
            None => Err(Error::new(
                format!("command {} terminated by signal", command),
                String::from(""),
            )),
        }
    }
}

/// Execute command only
#[cfg_attr(tarpaulin, skip)]
pub fn execute_command_stderr(w: &mut StandardStream, command: &str) -> Result<(), Error> {
    let mut cmd = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .arg("/C")
            .arg(command)
            .stderr(Stdio::piped())
            .spawn()?
    } else {
        Command::new("bash")
            .arg("-c")
            .arg(command)
            .stderr(Stdio::piped())
            .spawn()?
    };

    let stdout = cmd.stderr.as_mut().unwrap();
    let stdout_reader = BufReader::new(stdout);

    for line in stdout_reader.lines() {
        write!(w, "{}\n", line?)?;
    }

    let e = cmd.wait()?;

    if e.success() {
        Ok(())
    } else {
        match e.code() {
            Some(code) => Err(Error::new(
                format!("command {} failed with exit code {}", command, code),
                String::from(""),
            )),
            None => Err(Error::new(
                format!("command {} terminated by signal", command),
                String::from(""),
            )),
        }
    }
}
