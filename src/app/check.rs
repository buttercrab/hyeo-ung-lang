use crate::core::code::Code;
use crate::core::parse;
use crate::util::error::Error;
use crate::util::option::HyeongOption;
use crate::util::{io, option};
use clap::App;
use std::cmp::max;
use std::io::Write;
use termcolor::{Color, ColorSpec, StandardStream, WriteColor};

#[cfg_attr(tarpaulin, skip)]
pub fn app<'a, 'b>() -> App<'a, 'b> {
    App::new("check")
        .about("Parse your code and check if you are right")
        .arg(option::color())
        .arg(option::input())
}

#[cfg_attr(tarpaulin, skip)]
pub fn run(stdout: &mut StandardStream, hy_opt: HyeongOption) -> Result<(), Error> {
    let un_opt_code = io::parse_file(stdout, &hy_opt.input.as_ref().unwrap())?;
    let file_name = hy_opt
        .input
        .as_ref()
        .unwrap()
        .file_name()
        .unwrap()
        .clone()
        .to_os_string()
        .into_string()
        .map_err(|_| {
            Error::new(
                String::from("error on OsString to String conversion"),
                String::from("maybe the path is not correct"),
            )
        })?;
    let mut cnt = 0;
    let idx_len = un_opt_code.len().to_string().len();
    let mut file_len = 0usize;

    for c in un_opt_code.iter() {
        file_len = max(
            file_len,
            c.get_location().0.to_string().len() + c.get_location().1.to_string().len(),
        )
    }

    for c in un_opt_code.iter() {
        cnt += 1;

        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)))?;
        write!(stdout, "{}", cnt)?;
        stdout.reset()?;
        write!(
            stdout,
            "{} | ",
            std::iter::repeat(' ')
                .take(idx_len - cnt.to_string().len())
                .collect::<String>()
        )?;

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
            std::iter::repeat(' ')
                .take(
                    file_len
                        - c.get_location().0.to_string().len()
                        - c.get_location().1.to_string().len()
                )
                .collect::<String>()
        )?;

        writeln!(
            stdout,
            "{}_{}_{} {}",
            parse::COMMANDS[c.get_type() as usize],
            c.get_hangul_count(),
            c.get_dot_count(),
            c.get_area()
        )?;
    }

    Ok(())
}
