use clap::*;

use hyeong::{interpreter, io};

#[cfg_attr(tarpaulin, skip)]
fn main() {
    let matches = App::new("hyeong")
        .version("0.1.0")
        .about("Hyeo-ung programming language tool")
        .subcommand(
            App::new("build")
                .about("Compiles hyeong code")
                .arg(
                    Arg::with_name("input")
                        .value_name("input_file")
                        .takes_value(true)
                        .required(true)
                        .help("input file to compile")
                )
                .arg(
                    Arg::with_name("optimize")
                        .value_name("optimize")
                        .takes_value(true)
                        .short('O')
                        .long("optimize")
                        .help("optimize level (0: no optimize, 1: basic optimize, 2: hard optimize [default])")
                )
                .arg(
                    Arg::with_name("output")
                        .value_name("output")
                        .takes_value(true)
                        .short('o')
                        .long("output")
                        .help("binary output file (filename by default)")
                )
                .arg(
                    Arg::with_name("warning")
                        .value_name("warning")
                        .takes_value(true)
                        .short('W')
                        .long("warn")
                        .help("warning level (0/none: no warning, 1/all: all warning [default]")
                )
        )
        .subcommand(
            App::new("check")
                .about("Parse your code and check if you are right")
                .arg(
                    Arg::with_name("input")
                        .value_name("input_file")
                        .takes_value(true)
                        .required(true)
                        .help("input file to check")
                )
        )
        .subcommand(
            App::new("debug")
                .about("Debug your code command by command")
                .arg(
                    Arg::with_name("input")
                        .value_name("input_file")
                        .takes_value(true)
                        .required(true)
                        .help("input file to debug")
                )
                .arg(
                    Arg::with_name("from")
                        .value_name("from")
                        .takes_value(true)
                        .short('f')
                        .long("from")
                        .help("place to start debugging from (0 by default)")
                )
        )
        .subcommand(
            App::new("run")
                .about("Run hyeong code directly")
                .arg(
                    Arg::with_name("input")
                        .value_name("input_file")
                        .takes_value(true)
                        .required(true)
                        .help("input file to run")
                )
                .arg(
                    Arg::with_name("optimize")
                        .value_name("optimize")
                        .takes_value(true)
                        .short('O')
                        .long("optimize")
                        .help("optimize level (0: no optimize, 1: basic optimize, 2: hard optimize [default])")
                )
                .arg(
                    Arg::with_name("warning")
                        .value_name("warning")
                        .takes_value(true)
                        .short('W')
                        .long("warn")
                        .help("warning level (0/none: no warning, 1/all: all warning [default]")
                )
        )
        .subcommand(
            App::new("update")
                .about("Update this tool")
                .arg(
                    Arg::with_name("version")
                        .value_name("version")
                        .takes_value(true)
                        .help("update to specific version (latest by default)")
                )
        )
        .get_matches();

    if let Some(ref matches) = matches.subcommand_matches("build") {
        // build
        let code = io::read_file(matches.value_of("input").unwrap());
    } else if let Some(ref matches) = matches.subcommand_matches("check") {
        // check
        let file = matches.value_of("input").unwrap();
        let code = io::read_file(file);
        for c in code.iter() {
            println!("{}:{}", file, c.to_string())
        }
    } else if let Some(ref matches) = matches.subcommand_matches("debug") {
        // debug
        let code = io::read_file(matches.value_of("input").unwrap());
    } else if let Some(ref matches) = matches.subcommand_matches("run") {
        // run
        let code = io::read_file(matches.value_of("input").unwrap());
    } else if let Some(ref matches) = matches.subcommand_matches("update") {
        // update
        let version = if let Some(t) = matches.value_of("version") {
            t
        } else {
            "latest"
        };
    } else {
        // interpreter
        interpreter::Interpreter::new().run();
    }
}
