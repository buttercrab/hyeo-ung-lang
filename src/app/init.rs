use crate::util::error::Error;
use crate::util::option::HyeongOption;
use crate::util::{io, option, util};
use clap::App;
use std::fs;
use termcolor::{ColorChoice, StandardStream};

pub fn install_app<'a, 'b>() -> App<'a, 'b> {
    App::new("install")
        .about("Install core before build (need once)")
        .arg(option::build_source())
}

pub fn uninstall_app<'a, 'b>() -> App<'a, 'b> {
    App::new("uninstall")
        .about("Uninstall core temporary build path")
        .arg(option::build_source())
}

pub fn install_run(stdout: &mut StandardStream, hy_opt: HyeongOption) -> Result<(), Error> {
    io::print_log(stdout, "making dir for building core")?;
    io::save_to_file(
        &hy_opt.build_source.as_ref().unwrap().join("src/main.rs"),
        String::from(
            "\
use core::number::Num;

fn main() {
    let a = Num::from_num(10);
    println!(\"a = {}\", a);
}
",
        ),
    )?;
    io::save_to_file(
        &hy_opt.build_source.as_ref().unwrap().join("Cargo.toml"),
        String::from(
            "\
[package]
name = \"core-build\"
version = \"0.1.0\"

[dependencies]
core = \"0.1.0\"
",
        ),
    )?;
    io::print_log(stdout, "test build")?;
    io::execute_command_stderr(
        stdout,
        &*format!(
            "cargo build --manifest-path={} --release --color {}",
            util::path_to_string(&hy_opt.build_source.as_ref().unwrap())?,
            match hy_opt.color {
                ColorChoice::Always => "always",
                ColorChoice::AlwaysAnsi => "always",
                ColorChoice::Auto => "auto",
                ColorChoice::Never => "none",
            }
        ),
    )?;
    io::print_log(stdout, "done!")?;
    io::print_note(stdout, "to uninstall, run `core uninstall`")?;
    Ok(())
}

pub fn uninstall_run(stdout: &mut StandardStream, hy_opt: HyeongOption) -> Result<(), Error> {
    io::print_log(stdout, "removing dir")?;
    fs::remove_dir_all(hy_opt.build_source.as_ref().unwrap())?;
    io::print_log(stdout, "done!")?;
    Ok(())
}
