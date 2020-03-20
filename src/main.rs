use clap::App;
use hyeong::{build, check, debug, init, interpreter, run};

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
fn main() {
    let matches = App::new("hyeong")
        .version("0.1.1-dev")
        .about("hyeo-ung programming language tool")
        .subcommand(build::app())
        .subcommand(check::app())
        .subcommand(debug::app())
        .subcommand(run::app())
        .subcommand(init::install_app())
        .subcommand(init::uninstall_app())
        .get_matches();

    if let Some(ref matches) = matches.subcommand_matches("build") {
        build::run(matches);
    } else if let Some(ref matches) = matches.subcommand_matches("check") {
        check::run(matches);
    } else if let Some(ref matches) = matches.subcommand_matches("debug") {
        debug::run(matches);
    } else if let Some(ref matches) = matches.subcommand_matches("run") {
        run::run(matches);
    } else if let Some(ref matches) = matches.subcommand_matches("install") {
        init::install_run(matches);
    } else if let Some(ref matches) = matches.subcommand_matches("uninstall") {
        init::uninstall_run(matches);
    } else {
        interpreter::run();
    }
}
