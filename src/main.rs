#[cfg(not(feature = "number"))]
use clap::{App, ArgMatches};
#[cfg(not(feature = "number"))]
use hyeong::app::{build, check, debug, init, run};
#[cfg(not(feature = "number"))]
use hyeong::core::interpreter;
#[cfg(not(feature = "number"))]
use hyeong::util::error::Error;
#[cfg(not(feature = "number"))]
use hyeong::util::option::HyeongOption;
#[cfg(not(feature = "number"))]
use hyeong::util::{io, option};
#[cfg(not(feature = "number"))]
use termcolor::{ColorChoice, StandardStream};

#[cfg_attr(tarpaulin, skip)]
#[cfg(not(feature = "number"))]
fn sub_main(
    stdout: &mut StandardStream,
    stderr: &mut StandardStream,
    matches: ArgMatches,
    color: ColorChoice,
) -> Result<(), Error> {
    if let Some(ref matches) = matches.subcommand_matches("build") {
        let input = option::parse_input(matches)?;
        let output = option::parse_output(matches, &input)?;
        build::run(
            stdout,
            HyeongOption {
                build_source: Some(option::parse_build_source(matches)?),
                color,
                input: Some(input),
                optimize: option::parse_optimize(matches)?,
                output: Some(output),
            },
        )
    } else if let Some(ref matches) = matches.subcommand_matches("check") {
        check::run(
            stdout,
            HyeongOption {
                build_source: None,
                color,
                input: Some(option::parse_input(matches)?),
                optimize: 0,
                output: None,
            },
        )
    } else if let Some(ref matches) = matches.subcommand_matches("debug") {
        debug::run(
            stdout,
            HyeongOption {
                build_source: None,
                color,
                input: Some(option::parse_input(matches)?),
                optimize: 0,
                output: None,
            },
        )
    } else if let Some(ref matches) = matches.subcommand_matches("run") {
        run::run(
            stdout,
            stderr,
            HyeongOption {
                build_source: None,
                color,
                input: Some(option::parse_input(matches)?),
                optimize: option::parse_optimize(matches)?,
                output: None,
            },
        )
    } else if let Some(ref matches) = matches.subcommand_matches("install") {
        init::install_run(
            stdout,
            HyeongOption {
                build_source: Some(option::parse_build_source(matches)?),
                color,
                input: None,
                optimize: 0,
                output: None,
            },
        )
    } else if let Some(ref matches) = matches.subcommand_matches("uninstall") {
        init::uninstall_run(
            stdout,
            HyeongOption {
                build_source: Some(option::parse_build_source(matches)?),
                color,
                input: None,
                optimize: 0,
                output: None,
            },
        )
    } else {
        interpreter::run()
    }
}

/// Main function of this program
///
/// ```text
/// core 0.1.1-dev
/// hyeo-ung programming language tool
///
/// USAGE:
///     core [SUBCOMMAND]
///
/// FLAGS:
///     -h, --help       Prints help information
///     -V, --version    Prints version information
///
/// SUBCOMMANDS:
///     build        Compiles core code
///     check        Parse your code and check if you are right
///     debug        Debug your code command by command
///     help         Prints this message or the help of the given subcommand(s)
///     install      Install core before build (need once)
///     run          Run core code directly
///     uninstall    Uninstall core before build
/// ```
#[cfg_attr(tarpaulin, skip)]
#[cfg(not(feature = "number"))]
fn main() {
    let matches = App::new("core")
        .version("0.1.1-dev")
        .about("hyeo-ung programming language tool")
        .arg(option::color())
        .subcommand(build::app())
        .subcommand(check::app())
        .subcommand(debug::app())
        .subcommand(run::app())
        .subcommand(init::install_app())
        .subcommand(init::uninstall_app())
        .get_matches();

    let mut temp_stderr = StandardStream::stderr(ColorChoice::Auto);
    let color = io::handle(&mut temp_stderr, option::parse_color(&matches));
    let mut stdout = StandardStream::stdout(color);
    let mut stderr = StandardStream::stderr(color);
    let mut stderr_copy = StandardStream::stderr(color);

    io::handle(
        &mut stderr,
        sub_main(&mut stdout, &mut stderr_copy, matches, color),
    );
}

#[cfg_attr(tarpaulin, skip)]
#[cfg(feature = "number")]
fn main() {}
