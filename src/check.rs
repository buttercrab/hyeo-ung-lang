use crate::{io, option};
use clap::{App, ArgMatches};

#[cfg_attr(tarpaulin, skip)]
pub fn app<'a, 'b>() -> App<'a, 'b> {
    App::new("check")
        .about("Parse your code and check if you are right")
        .arg(option::input())
}

pub fn run(matches: &ArgMatches) {
    let file = matches.value_of("input").unwrap();
    let code = io::read_file(file);
    for c in code.iter() {
        println!("{}:{}", file, c.to_string())
    }
}
