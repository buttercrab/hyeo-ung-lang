use crate::util::error::Error;
use crate::util::option::HyeongOption;
use crate::util::{io, option};
use clap::App;
use termcolor::StandardStream;

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

    for c in un_opt_code.iter() {
        println!("{}:{}", 1, c.to_string())
    }

    Ok(())
}
