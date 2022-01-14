use crate::core::code::UnOptCode;
use crate::core::parse;
use crate::number::num::Num;
use crate::util::error::Error;
use crate::util::io;
use crate::util::option::HyeongOption;
use std::io::Write;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::{Command, Stdio};
use termcolor::StandardStream;

/// PathBuf to String helper function
///
/// # Examples
///
/// ```
/// use hyeong::util::ext;
/// use std::path::PathBuf;
///
/// assert_eq!("foo/bar.hyeong", ext::path_to_string(&PathBuf::from("foo/bar.hyeong")).unwrap());
/// ```
pub fn path_to_string(path: &Path) -> Result<String, Error> {
    path.as_os_str().to_os_string().into_string().map_err(|_| {
        Error::new(
            "error on OsString to String conversion",
            "maybe the path is not correct",
        )
    })
}

/// Read and parse file
///
/// # Examples
///
/// ```
/// use termcolor::{StandardStream, ColorChoice};
/// use hyeong::util::ext;
/// use std::path::PathBuf;
/// use hyeong::util::option::HyeongOption;
///
/// let mut s = StandardStream::stdout(ColorChoice::Auto);
/// let c = ext::parse_file(&mut s, &PathBuf::from("examples/hello_world/hello_world.hyeong"), &HyeongOption::new()).unwrap();
///
/// assert_eq!(44, c.len());
/// ```
pub fn parse_file(
    stdout: &mut StandardStream,
    path: &Path,
    option: &HyeongOption,
) -> Result<Vec<UnOptCode>, Error> {
    let raw_code = io::read_file(path)?;
    io::print_log(stdout, format!("parsing {}", path_to_string(path)?))?;
    let un_opt_code = parse::parse(raw_code);
    if option.verbose {
        io::print_log(stdout, format!("⮑  total {} commands", un_opt_code.len()))?;
    }
    Ok(un_opt_code)
}

/// change `Num` to unicode char
///
/// # Examples
///
/// ```
/// use hyeong::number::num::Num;
/// use hyeong::util::ext;
///
/// let a = Num::from_num(55357);
/// let b = Num::from_num(0xAC00);
///
/// assert!(ext::num_to_unicode(&a).is_err());
/// assert!(matches!(ext::num_to_unicode(&b), Ok('가')));
/// ```
pub fn num_to_unicode(num: &Num) -> Result<char, Error> {
    let n = num.floor().to_int();
    std::char::from_u32(n).ok_or_else(|| {
        Error::new(
            "utf-8 encoding error",
            format!("number {} is not valid unicode", n),
        )
    })
}

/// Execute command and stream stdout to `StandardStream`
#[cfg(not(tarpaulin_include))]
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
        writeln!(w, "{}", line?)?;
    }

    let e = cmd.wait()?;

    if e.success() {
        Ok(())
    } else {
        match e.code() {
            Some(code) => Err(Error::new(
                format!("command {} failed with exit code {}", command, code),
                "",
            )),
            None => Err(Error::new(
                format!("command {} terminated by signal", command),
                "",
            )),
        }
    }
}

/// Execute command and stream stdout to `StandardStream`
#[cfg(not(tarpaulin_include))]
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
        writeln!(w, "{}", line?)?;
    }

    let e = cmd.wait()?;

    if e.success() {
        Ok(())
    } else {
        match e.code() {
            Some(code) => Err(Error::new(
                format!("command {} failed with exit code {}", command, code),
                "",
            )),
            None => Err(Error::new(
                format!("command {} terminated by signal", command),
                "",
            )),
        }
    }
}
