use crate::util::error::Error;
use clap::{Arg, ArgMatches};
use std::env;
use std::path::PathBuf;
use termcolor::ColorChoice;

#[cfg_attr(tarpaulin, skip)]
pub fn build_path<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("build-path")
        .value_name("build-path")
        .takes_value(true)
        .long("build-path")
        .help("set temporary build path")
        .multiple(false)
}

#[cfg_attr(tarpaulin, skip)]
pub fn parse_build_path(matches: &ArgMatches) -> std::io::Result<PathBuf> {
    if let Some(t) = matches.value_of("build-path") {
        let p = PathBuf::from(t);
        if !p.is_absolute() {
            let mut abs = env::current_dir()?;
            abs.push(p);
            Ok(abs)
        } else {
            Ok(p)
        }
    } else {
        let mut p = if cfg!(target_os = "windows") {
            PathBuf::from(&env::var("USERPROFILE").unwrap().replace("\\", "/"))
        } else {
            PathBuf::from(&env::var("HOME").unwrap())
        };
        p.push(".hyeong");
        Ok(p)
    }
}

#[cfg_attr(tarpaulin, skip)]
pub fn color<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("color")
        .value_name("color")
        .takes_value(true)
        .long("color")
        .help("whether prints color (never, auto, always)")
        .default_value("auto")
        .global(true)
        .possible_values(&["never", "auto", "always"])
        .multiple(false)
}

#[cfg_attr(tarpaulin, skip)]
pub fn parse_color(matches: &ArgMatches) -> Result<ColorChoice, Error> {
    match matches.value_of("color").unwrap() {
        "never" => Ok(ColorChoice::Never),
        "auto" => Ok(ColorChoice::Auto),
        "always" => Ok(ColorChoice::Always),
        t => Err(Error::new(
            format!("color has no option {}", t),
            String::from("options are: (never, auto, always)"),
        )),
    }
}

#[cfg_attr(tarpaulin, skip)]
pub fn input<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("input")
        .value_name("FILE.hyeong")
        .takes_value(true)
        .required(true)
        .help("input file to compile")
        .multiple(false)
}

#[cfg_attr(tarpaulin, skip)]
pub fn parse_input(matches: &ArgMatches) -> std::io::Result<PathBuf> {
    let p = PathBuf::from(matches.value_of("input").unwrap());
    if !p.is_absolute() {
        let mut abs = env::current_dir()?;
        abs.push(p);
        Ok(abs)
    } else {
        Ok(p)
    }
}

#[cfg_attr(tarpaulin, skip)]
pub fn optimize<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("optimize")
        .value_name("optimize")
        .takes_value(true)
        .short("O")
        .long("optimize")
        .help("optimize level (0: no optimize, 1: basic optimize, 2: hard optimize)")
        .default_value("2")
        .possible_values(&["0", "1", "2"])
        .multiple(false)
}

#[cfg_attr(tarpaulin, skip)]
pub fn parse_optimize(matches: &ArgMatches) -> Result<u8, Error> {
    match matches.value_of("optimize").unwrap() {
        "0" => Ok(0),
        "1" => Ok(1),
        "2" => Ok(2),
        t => Err(Error::new(
            format!("optimize has no option {}", t),
            String::from(""),
        )),
    }
}

#[cfg_attr(tarpaulin, skip)]
pub fn output<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("output")
        .value_name("output")
        .takes_value(true)
        .short("o")
        .long("output")
        .help("binary output file (filename by default)")
        .multiple(false)
}

#[cfg_attr(tarpaulin, skip)]
pub fn parse_output(matches: &ArgMatches, input: &PathBuf) -> std::io::Result<PathBuf> {
    if let Some(t) = matches.value_of("output") {
        let p = PathBuf::from(t);
        if !p.is_absolute() {
            let mut abs = env::current_dir()?;
            abs.push(p);
            Ok(abs)
        } else {
            Ok(p)
        }
    } else {
        let mut p = input.clone();
        p.set_extension("");
        Ok(p)
    }
}

#[derive(Clone)]
pub struct HyeongOption {
    pub build_source: Option<PathBuf>,
    pub color: ColorChoice,
    pub input: Option<PathBuf>,
    pub optimize: u8,
    pub output: Option<PathBuf>,
}
