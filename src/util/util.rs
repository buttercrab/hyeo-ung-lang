use crate::util::error::Error;
use std::io::Write;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use termcolor::StandardStream;

pub fn path_to_string(path: &PathBuf) -> Result<String, Error> {
    path.clone().into_os_string().into_string().map_err(|_| {
        Error::new(
            "error on OsString to String conversion",
            "maybe the path is not correct",
        )
    })
}

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
                "",
            )),
            None => Err(Error::new(
                format!("command {} terminated by signal", command),
                "",
            )),
        }
    }
}

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
                "",
            )),
            None => Err(Error::new(
                format!("command {} terminated by signal", command),
                "",
            )),
        }
    }
}
