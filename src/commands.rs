use crate::error_barrier;
use crate::hyeong::code::{Code, UnOptCode};
use crate::hyeong::execute::ExecutableState;
use crate::hyeong::optimize::optimize;
use crate::hyeong::parse::{parse, Span};
use crate::hyeong::state::UnOptState;
use anyhow::{bail, Result};
use colored::Colorize;
use log::{debug, error, info, warn};
use nom::error::ErrorKind;
use std::ffi::OsString;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

fn read_file(path: &PathBuf) -> Result<String> {
    let path_str = path.display().to_string();
    debug!(target: "Checking", "{}", path_str.underline());
    if !matches!(path.extension().map(|x| x.to_str()), Some(Some("hyeong"))) {
        bail!("file extension must be .hyeong");
    }
    let mut file = File::open(path)?;
    debug!(target: "Reading", "{}", path_str.underline());
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    debug!(target: "Read", "{}, total {} byte(s)", path_str.underline(), contents.len().to_string().underline());
    Ok(contents)
}

fn parse_file<'a>(code: &'a str, path: &Path) -> Result<Vec<UnOptCode<'a>>> {
    let path_str = path.display().to_string();
    info!(target: "Parsing", "{}", path_str.underline());
    let code = match parse::<(Span, ErrorKind)>(code) {
        Ok(code) => code,
        Err(err) => {
            error!("{}", err);
            error_barrier(format_args!("cannot compile due to previous error"));
            unreachable!();
        }
    };
    debug!(target: "Parsed", "{}, total {} command(s)", path_str.underline(), code.len().to_string().underline());
    Ok(code)
}

#[inline]
fn number_len(mut num: usize) -> usize {
    let mut len = (num == 0) as usize;
    while num > 0 {
        num /= 10;
        len += 1;
    }
    len
}

fn print_one_code(i: usize, c: &UnOptCode, idx_len: usize, file_name: &str, file_len: usize, raw: bool) {
    let i = (i + 1).to_string().blue().bold();
    let (line, col) = c.location();
    let file_len = file_len - number_len(line) - number_len(col);
    let line = line.to_string().yellow();
    let col = col.to_string().yellow();
    const SPACES: &str = "";

    if raw {
        info!(target: "", "{i:>idx_len$} {file_name}:{line}:{col}{SPACES:file_len$}  {}", c.raw());
    } else {
        info!(target: "", "{i:>idx_len$} {file_name}:{line}:{col}{SPACES:file_len$}  {}_{}_{} {}", c.type_(), c.hangul_count(), c.area_count(), c.area());
    }
}

pub fn build(level: u8, file: PathBuf) -> Result<()> {
    let code = read_file(&file)?;
    let un_opt_code = parse_file(&code, &file)?;
    info!(target: "Building", "file: {}", file.display());
    let rust_code = if level >= 1 {
        debug!(target: "Optimizing", "level {}", level);
        let (_state, _code) = optimize(un_opt_code, level)?;
        "".to_string()
        // build_source(state, &code, level)
    } else {
        let _state = UnOptState::new();
        "".to_string()
        // build_source(state, &un_opt_code, level)
    };
    debug!(target: "Building", "rust code, total {} byte(s)", rust_code.len());
    // todo: finish build
    Ok(())
}

pub fn check(file: PathBuf, raw: bool) -> Result<()> {
    let code = read_file(&file)?;
    let un_opt_code = parse_file(&code, &file)?;
    let file_name_warn = |_| {
        warn!("error on OsString to String conversion, maybe the path is not correct");
        "<file>".to_string()
    };
    let file_name = if let Some(file_name) = file.file_name() {
        file_name
            .to_os_string()
            .into_string()
            .unwrap_or_else(file_name_warn)
    } else {
        file_name_warn(OsString::new())
    };
    let idx_len = number_len(un_opt_code.len() + 1) + 1;
    let file_len = un_opt_code
        .iter()
        .map(|c| c.location())
        .map(|(line, col)| number_len(line) + number_len(col))
        .max()
        .unwrap_or(0);

    un_opt_code.into_iter().enumerate().for_each(|(i, c)| {
        print_one_code(i, &c, idx_len, &file_name, file_len, raw);
    });
    Ok(())
}

pub fn debug(_file: PathBuf) -> Result<()> {
    todo!()
}

pub fn run(level: u8, file: PathBuf) -> Result<()> {
    let code = read_file(&file)?;
    let un_opt_code = parse_file(&code, &file)?;
    info!(target: "Building", "file: {}", file.display());
    let mut stdin = std::io::stdin().lock();
    let mut stdout = std::io::stdout().lock();
    let mut stderr = std::io::stderr().lock();

    if level >= 1 {
        debug!(target: "Optimizing", "level {}", level);
        let (mut state, code) = optimize(un_opt_code, level)?;
        code.into_iter()
            .try_for_each(|c| state.execute(&mut stdin, &mut stdout, &mut stderr, &c))?;
    } else {
        let mut state = UnOptState::new();
        un_opt_code
            .into_iter()
            .try_for_each(|c| state.execute(&mut stdin, &mut stdout, &mut stderr, &c))?;
    }

    Ok(())
}

pub fn interpret() -> Result<()> {
    todo!()
}
