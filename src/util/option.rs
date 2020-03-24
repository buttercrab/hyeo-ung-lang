use crate::util::error::Error;
use clap::{Arg, ArgMatches};
use std::env;
use std::path::PathBuf;
use termcolor::ColorChoice;

/// Path to temporarily build compiled rust code
#[cfg_attr(tarpaulin, skip)]
pub fn build_path<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("build-path")
        .value_name("build-path")
        .takes_value(true)
        .long("build-path")
        .help("set temporary build path")
        .multiple(false)
}

/// Parse build path option
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

/// Color option
#[cfg_attr(tarpaulin, skip)]
pub fn color<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("color")
        .value_name("color")
        .takes_value(true)
        .long("color")
        .help("whether prints color")
        .default_value("auto")
        .global(true)
        .possible_values(&["never", "auto", "always"])
        .multiple(false)
}

/// Parse color option
#[cfg_attr(tarpaulin, skip)]
pub fn parse_color(matches: &ArgMatches) -> Result<ColorChoice, Error> {
    match matches.value_of("color").unwrap() {
        "never" => Ok(ColorChoice::Never),
        "auto" => Ok(ColorChoice::Auto),
        "always" => Ok(ColorChoice::Always),
        _ => unreachable!(),
    }
}

/// Path to input of program
#[cfg_attr(tarpaulin, skip)]
pub fn input<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("input")
        .value_name("FILE.hyeong")
        .takes_value(true)
        .required(true)
        .help("input file to compile")
        .multiple(false)
}

/// Parse input and make to absolute path
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

/// Optimization option
#[cfg_attr(tarpaulin, skip)]
pub fn optimize<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("optimize")
        .value_name("optimize")
        .takes_value(true)
        .short("O")
        .long("optimize")
        .help("optimize level")
        .default_value("2")
        .possible_values(&["0", "1", "2"])
        .multiple(false)
}

/// Parse optimization
#[cfg_attr(tarpaulin, skip)]
pub fn parse_optimize(matches: &ArgMatches) -> Result<u8, Error> {
    match matches.value_of("optimize").unwrap() {
        "0" => Ok(0),
        "1" => Ok(1),
        "2" => Ok(2),
        _ => unreachable!(),
    }
}

/// Path to output of program
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

/// Parse output and make to absolute path
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

/// verbose option
#[cfg_attr(tarpaulin, skip)]
pub fn verbose<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("verbose")
        .value_name("verbose")
        .long("verbose")
        .takes_value(false)
        .required(false)
        .help("verbose output")
        .multiple(false)
}

/// Parse verbose flag
#[cfg_attr(tarpaulin, skip)]
pub fn parse_verbose(matches: &ArgMatches) -> bool {
    matches.is_present("verbose")
}

/// All of the options
#[derive(Clone)]
pub struct HyeongOption {
    pub build_path: Option<PathBuf>,
    pub color: ColorChoice,
    pub input: Option<PathBuf>,
    pub optimize: u8,
    pub output: Option<PathBuf>,
    pub verbose: bool,
}

impl HyeongOption {
    /// New Hyeong option
    #[cfg_attr(tarpaulin, skip)]
    pub fn new() -> HyeongOption {
        HyeongOption {
            build_path: None,
            color: ColorChoice::Auto,
            input: None,
            optimize: 0,
            output: None,
            verbose: false,
        }
    }

    /// Add `build_path` option
    #[cfg_attr(tarpaulin, skip)]
    pub fn build_path(mut self, path: PathBuf) -> HyeongOption {
        self.build_path = Some(path);
        self
    }

    /// Add `color` option
    #[cfg_attr(tarpaulin, skip)]
    pub fn color(mut self, color: ColorChoice) -> HyeongOption {
        self.color = color;
        self
    }

    /// Add `input` option
    #[cfg_attr(tarpaulin, skip)]
    pub fn input(mut self, path: PathBuf) -> HyeongOption {
        self.input = Some(path);
        self
    }

    /// Add `optimize` option
    #[cfg_attr(tarpaulin, skip)]
    pub fn optimize(mut self, level: u8) -> HyeongOption {
        self.optimize = level;
        self
    }

    /// Add `output` option
    #[cfg_attr(tarpaulin, skip)]
    pub fn output(mut self, path: PathBuf) -> HyeongOption {
        self.output = Some(path);
        self
    }

    /// Add `verbose` option
    #[cfg_attr(tarpaulin, skip)]
    pub fn verbose(mut self, verbose: bool) -> HyeongOption {
        self.verbose = verbose;
        self
    }
}
