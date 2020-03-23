#[cfg(not(feature = "number"))]
use clap::{App, ArgMatches};
#[cfg(not(feature = "number"))]
use hyeong::app::{build, check, debug, init, interpreter, run};
#[cfg(not(feature = "number"))]
use hyeong::util::{error::Error, io, option, option::HyeongOption};
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
            &HyeongOption {
                build_source: Some(option::parse_build_path(matches)?),
                color,
                input: Some(input),
                optimize: option::parse_optimize(matches)?,
                output: Some(output),
            },
        )
    } else if let Some(ref matches) = matches.subcommand_matches("check") {
        check::run(
            stdout,
            &HyeongOption {
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
            &HyeongOption {
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
            &HyeongOption {
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
            &HyeongOption {
                build_source: Some(option::parse_build_path(matches)?),
                color,
                input: None,
                optimize: 0,
                output: None,
            },
        )
    } else if let Some(ref matches) = matches.subcommand_matches("uninstall") {
        init::uninstall_run(
            stdout,
            &HyeongOption {
                build_source: Some(option::parse_build_path(matches)?),
                color,
                input: None,
                optimize: 0,
                output: None,
            },
        )
    } else {
        interpreter::run(
            stdout,
            &HyeongOption {
                build_source: None,
                color,
                input: None,
                optimize: 0,
                output: None,
            },
        )
    }
}

/// Main function of this program
///
/// ```text
/// hyeong 0.1.1-dev
/// hyeo-ung programming language tool
///
/// USAGE:
///     hyeong [SUBCOMMAND]
///
/// FLAGS:
///     -h, --help       Prints help information
///     -V, --version    Prints version information
///
/// SUBCOMMANDS:
///     build        Compiles hyeong code
///     check        Parse your code and check if you are right
///     debug        Debug your code command by command
///     help         Prints this message or the help of the given subcommand(s)
///     install      Install hyeong before build (need once)
///     run          Run hyeong code directly
///     uninstall    Uninstall hyeong before build
/// ```
#[cfg_attr(tarpaulin, skip)]
#[cfg(not(feature = "number"))]
fn main() {
    let matches = App::new("hyeong")
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
