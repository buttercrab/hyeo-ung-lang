use crate::core::code::{Code, UnOptCode};
use crate::core::compile::build_source;
use crate::core::optimize::optimize;
use crate::core::parse;
use crate::core::parse::parse;
use crate::core::state::UnOptState;
use anyhow::{bail, Result};
use colored::Colorize;
use log::{debug, error, info, warn};
use std::cmp::max;
use std::ffi::OsString;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

fn read_file(path: PathBuf) -> Result<String> {
    let path_str = path.display().to_string();
    debug!(target: "Checking", "{}", path_str.to_string().underline());
    if !matches!(path.extension().map(|x| x.to_str()), Some(Some("hyeong"))) {
        bail!("file extension must be .hyeong");
    }
    let mut file = File::open(path.clone())?;
    debug!(target: "Opening", "{}", path_str.to_string().underline());
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    debug!(target: "Read", "{}, total {} byte(s)", path_str.to_string().underline(), contents.len().to_string().underline());
    Ok(contents)
}

fn parse_file(path: PathBuf) -> Result<Vec<UnOptCode>> {
    let path_str = path.display();
    let file = read_file(path.clone())?;
    info!(target: "Parsing", "{}", path_str.to_string().underline());
    let code = parse(file);
    debug!(target: "Parsed", "{}, total {} command(s)", path_str.to_string().underline(), code.len().to_string().underline());
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

pub fn build(level: u8, file: PathBuf) -> Result<()> {
    let un_opt_code = parse_file(file.clone())?;
    info!(target: "Building", "file: {}", file.display());
    debug!(target: "Optimizing", "level {}", level);
    let rust_code = if level >= 1 {
        let (state, code) = optimize(un_opt_code, level)?;
        build_source(state, &code, level)
    } else {
        let state = UnOptState::new();
        build_source(state, &un_opt_code, level)
    };
    debug!(target: "Building", "rust code, total {} byte(s)", rust_code.len());
    // todo: finish build
    Ok(())
}

pub fn check(file: PathBuf, raw: bool) -> Result<()> {
    let un_opt_code = parse_file(file.clone())?;
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

    for (i, c) in un_opt_code.iter().enumerate() {
        let i = (i + 1).to_string().blue().bold();
        let (line, col) = c.location();
        let file_len = file_len - number_len(line) - number_len(col);
        let line = line.to_string().yellow();
        let col = col.to_string().yellow();
        const SPACES: &str = "";

        let code = if raw {
            format!("{}", c.raw())
        } else {
            format!(
                "{}_{}_{} {}",
                parse::COMMANDS[c.type_() as usize],
                c.hangul_count(),
                c.area_count(),
                c.area()
            )
        };

        info!(target: "", "{i:>idx_len$} {file_name}:{line}:{col}{SPACES:file_len$}  {code}");
    }
    Ok(())
}

pub fn debug(file: PathBuf) -> Result<()> {
    todo!()
}

pub fn run(level: u8, file: PathBuf) -> Result<()> {
    todo!()
}

pub fn interpret() -> Result<()> {
    todo!()
}
