use std::cmp::max;
use std::io::Write;

use clap::App;
use termcolor::{Color, ColorSpec, StandardStream, WriteColor};

use crate::core::code::{Code, UnOptCode};
use crate::core::parse;
use crate::util::error::Error;
use crate::util::option::HyeongOption;
use crate::util::{ext, option};

/// App for check
#[cfg(not(tarpaulin_include))]
pub fn app<'a>() -> App<'a> {
    App::new("check")
        .about("Parse your code and check if you are right")
        .arg(option::input())
}

/// Runner for check
#[cfg(not(tarpaulin_include))]
pub fn run(stdout: &mut StandardStream, hy_opt: &HyeongOption) -> Result<(), Error> {
    let un_opt_code = ext::parse_file(stdout, hy_opt.input.as_ref().unwrap(), hy_opt)?;
    print_un_opt_codes(
        stdout,
        hy_opt,
        un_opt_code.iter().enumerate().collect::<Vec<_>>(),
        Color::Cyan,
        false,
    )
}

/// Main print function for `UnOptCode`
#[cfg(not(tarpaulin_include))]
pub fn print_un_opt_codes(
    stdout: &mut StandardStream,
    hy_opt: &HyeongOption,
    code: Vec<(usize, &UnOptCode)>,
    color: Color,
    raw: bool,
) -> Result<(), Error> {
    let file_name = hy_opt
        .input
        .as_ref()
        .unwrap()
        .file_name()
        .unwrap()
        .to_os_string()
        .into_string()
        .map_err(|_| {
            Error::new(
                "error on OsString to String conversion",
                "maybe the path is not correct",
            )
        })?;
    let mut idx_len = 0usize;
    let mut file_len = 0usize;

    for (i, c) in code.iter() {
        idx_len = max(idx_len, i.to_string().len());

        file_len = max(
            file_len,
            c.get_location().0.to_string().len() + c.get_location().1.to_string().len(),
        )
    }

    for (i, c) in code.iter() {
        stdout.set_color(ColorSpec::new().set_fg(Some(color)))?;
        write!(stdout, "{}", i)?;
        stdout.reset()?;
        write!(stdout, "{} | ", " ".repeat(idx_len - i.to_string().len()))?;

        write!(stdout, "{}:", file_name)?;
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))?;
        write!(stdout, "{}", c.get_location().0)?;
        stdout.reset()?;
        write!(stdout, ":")?;
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))?;
        write!(stdout, "{}", c.get_location().1)?;
        stdout.reset()?;

        write!(
            stdout,
            "{}  ",
            " ".repeat(
                file_len - c.get_location().0.to_string().len() - c.get_location().1.to_string().len()
            )
        )?;

        if raw {
            writeln!(stdout, "{}", c.get_raw())?;
        } else {
            writeln!(
                stdout,
                "{}_{}_{} {}",
                parse::COMMANDS[c.get_type() as usize],
                c.get_hangul_count(),
                c.get_dot_count(),
                c.get_area()
            )?;
        }
    }

    Ok(())
}
