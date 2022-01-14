use crate::util::error::Error;
use clap::{Arg, ArgMatches};
use std::env;
use std::path::PathBuf;
use termcolor::ColorChoice;

/// Path to temporarily build compiled rust code
#[cfg(not(tarpaulin_include))]
pub fn build_path<'a>() -> Arg<'a> {
    Arg::new("build-path")
        .value_name("build-path")
        .takes_value(true)
        .long("build-path")
        .help("set temporary build path")
        .multiple_occurrences(false)
}

/// Parse build path option
#[cfg(not(tarpaulin_include))]
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
            PathBuf::from(&env::var("USERPROFILE").unwrap().replace('\\', "/"))
        } else {
            PathBuf::from(&env::var("HOME").unwrap())
        };
        p.push(".hyeong");
        Ok(p)
    }
}

/// Color option
#[cfg(not(tarpaulin_include))]
pub fn color<'a>() -> Arg<'a> {
    Arg::new("color")
        .value_name("color")
        .takes_value(true)
        .long("color")
        .help("whether prints color")
        .default_value("auto")
        .global(true)
        .possible_values(&["never", "auto", "always"])
        .multiple_occurrences(false)
}

/// Parse color option
#[cfg(not(tarpaulin_include))]
pub fn parse_color(matches: &ArgMatches) -> Result<ColorChoice, Error> {
    match matches.value_of("color").unwrap() {
        "never" => Ok(ColorChoice::Never),
        "auto" => Ok(ColorChoice::Auto),
        "always" => Ok(ColorChoice::Always),
        _ => unreachable!(),
    }
}

/// Path to input of program
#[cfg(not(tarpaulin_include))]
pub fn input<'a>() -> Arg<'a> {
    Arg::new("input")
        .value_name("FILE.hyeong")
        .takes_value(true)
        .required(true)
        .help("input file to compile")
        .multiple_occurrences(false)
}

/// Parse input and make to absolute path
#[cfg(not(tarpaulin_include))]
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
#[cfg(not(tarpaulin_include))]
pub fn optimize<'a>() -> Arg<'a> {
    Arg::new("optimize")
        .value_name("optimize")
        .takes_value(true)
        .short('O')
        .long("optimize")
        .help("optimize level")
        .default_value("2")
        .possible_values(&["0", "1", "2"])
        .multiple_occurrences(false)
}

/// Parse optimization
#[cfg(not(tarpaulin_include))]
pub fn parse_optimize(matches: &ArgMatches) -> Result<u8, Error> {
    match matches.value_of("optimize").unwrap() {
        "0" => Ok(0),
        "1" => Ok(1),
        "2" => Ok(2),
        _ => unreachable!(),
    }
}

/// Path to output of program
#[cfg(not(tarpaulin_include))]
pub fn output<'a>() -> Arg<'a> {
    Arg::new("output")
        .value_name("output")
        .takes_value(true)
        .short('o')
        .long("output")
        .help("binary output file (filename by default)")
        .multiple_occurrences(false)
}

/// Parse output and make to absolute path
#[cfg(not(tarpaulin_include))]
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
#[cfg(not(tarpaulin_include))]
pub fn verbose<'a>() -> Arg<'a> {
    Arg::new("verbose")
        .value_name("verbose")
        .long("verbose")
        .global(true)
        .takes_value(false)
        .required(false)
        .help("verbose output")
        .multiple_occurrences(false)
}

/// Parse verbose flag
#[cfg(not(tarpaulin_include))]
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
    #[cfg(not(tarpaulin_include))]
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
    #[cfg(not(tarpaulin_include))]
    pub fn build_path(mut self, path: PathBuf) -> HyeongOption {
        self.build_path = Some(path);
        self
    }

    /// Add `color` option
    #[cfg(not(tarpaulin_include))]
    pub fn color(mut self, color: ColorChoice) -> HyeongOption {
        self.color = color;
        self
    }

    /// Add `input` option
    #[cfg(not(tarpaulin_include))]
    pub fn input(mut self, path: PathBuf) -> HyeongOption {
        self.input = Some(path);
        self
    }

    /// Add `optimize` option
    #[cfg(not(tarpaulin_include))]
    pub fn optimize(mut self, level: u8) -> HyeongOption {
        self.optimize = level;
        self
    }

    /// Add `output` option
    #[cfg(not(tarpaulin_include))]
    pub fn output(mut self, path: PathBuf) -> HyeongOption {
        self.output = Some(path);
        self
    }

    /// Add `verbose` option
    #[cfg(not(tarpaulin_include))]
    pub fn verbose(mut self, verbose: bool) -> HyeongOption {
        self.verbose = verbose;
        self
    }
}
