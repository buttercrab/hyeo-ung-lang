#[cfg(not(feature = "number"))]
use clap::{App, ArgMatches};
#[cfg(not(feature = "number"))]
use hyeong::app::{build, check, debug, init, interpreter, run};
#[cfg(not(feature = "number"))]
use hyeong::util::{error::Error, io, option, option::HyeongOption};
#[cfg(not(feature = "number"))]
use termcolor::{ColorChoice, StandardStream};

/// Main function that executes command
#[cfg_attr(tarpaulin, skip)]
#[cfg(not(feature = "number"))]
fn sub_main(
    stdout: &mut StandardStream,
    stderr: &mut StandardStream,
    matches: ArgMatches,
    hy_opt: HyeongOption,
) -> Result<(), Error> {
    if let Some(ref matches) = matches.subcommand_matches("build") {
        let input = option::parse_input(matches)?;
        let output = option::parse_output(matches, &input)?;
        build::run(
            stdout,
            &hy_opt
                .build_path(option::parse_build_path(matches)?)
                .input(input)
                .optimize(option::parse_optimize(matches)?)
                .output(output),
        )
    } else if let Some(ref matches) = matches.subcommand_matches("check") {
        check::run(stdout, &hy_opt.input(option::parse_input(matches)?))
    } else if let Some(ref matches) = matches.subcommand_matches("debug") {
        debug::run(stdout, &hy_opt.input(option::parse_input(matches)?))
    } else if let Some(ref matches) = matches.subcommand_matches("run") {
        run::run(
            stdout,
            stderr,
            &hy_opt
                .input(option::parse_input(matches)?)
                .optimize(option::parse_optimize(matches)?),
        )
    } else if let Some(ref matches) = matches.subcommand_matches("install") {
        init::install_run(
            stdout,
            &hy_opt.build_path(option::parse_build_path(matches)?),
        )
    } else if let Some(ref matches) = matches.subcommand_matches("uninstall") {
        init::uninstall_run(
            stdout,
            &hy_opt.build_path(option::parse_build_path(matches)?),
        )
    } else {
        interpreter::run(stdout, &hy_opt)
    }
}

/// Main function of this program
///
/// ```text
/// hyeong 0.1.3
/// hyeo-ung programming language tool
///
/// USAGE:
///     hyeong [FLAGS] [OPTIONS] [SUBCOMMAND]
///
/// FLAGS:
///     -h, --help       Prints help information
///     -V, --version    Prints version information
///         --verbose    verbose output
///
/// OPTIONS:
///         --color <color>    whether prints color [default: auto]  [possible values: never, auto, always]
///
/// SUBCOMMANDS:
///     build        Compiles hyeong code
///     check        Parse your code and check if you are right
///     debug        Debug your code command by command
///     help         Prints this message or the help of the given subcommand(s)
///     install      Install hyeong before build (need once)
///     run          Run hyeong code directly
///     uninstall    Uninstall hyeong temporary build path
/// ```
#[cfg_attr(tarpaulin, skip)]
#[cfg(not(feature = "number"))]
fn main() {
    let matches = App::new("hyeong")
        .version("0.1.3")
        .about("hyeo-ung programming language tool")
        .arg(option::color())
        .arg(option::verbose())
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
        sub_main(
            &mut stdout,
            &mut stderr_copy,
            matches.clone(),
            HyeongOption::new()
                .color(color)
                .verbose(option::parse_verbose(&matches)),
        ),
    );
}

#[cfg_attr(tarpaulin, skip)]
#[cfg(feature = "number")]
fn main() {}
