use crate::util::error::Error;
use crate::util::option::HyeongOption;
use crate::util::{io, option, util};
use clap::App;
use std::fs;
use termcolor::{StandardStream, WriteColor};

#[cfg_attr(tarpaulin, skip)]
pub fn install_app<'a, 'b>() -> App<'a, 'b> {
    App::new("install")
        .about("Install hyeong before build (need once)")
        .arg(option::build_path())
}

#[cfg_attr(tarpaulin, skip)]
pub fn uninstall_app<'a, 'b>() -> App<'a, 'b> {
    App::new("uninstall")
        .about("Uninstall hyeong temporary build path")
        .arg(option::build_path())
}

#[cfg_attr(tarpaulin, skip)]
pub fn install_run(stdout: &mut StandardStream, hy_opt: HyeongOption) -> Result<(), Error> {
    io::print_log(stdout, "making dir for building hyeong")?;
    fs::create_dir_all(
        &hy_opt
            .build_source
            .as_ref()
            .unwrap()
            .join("hyeong-build/src"),
    )?;
    io::save_to_file(
        &hy_opt
            .build_source
            .as_ref()
            .unwrap()
            .join("hyeong-build/src/main.rs"),
        String::from(
            "\
use hyeong::number::number::Num;

fn main() {
    let a = Num::from_num(10);
    println!(\"a = {}\", a);
}
",
        ),
    )?;
    io::save_to_file(
        &hy_opt
            .build_source
            .as_ref()
            .unwrap()
            .join("hyeong-build/Cargo.toml"),
        String::from(
            "\
[package]
name = \"hyeong-build\"
version = \"0.1.0\"
edition = \"2018\"

[dependencies]
hyeong = { git = \"https://github.com/buttercrab/hyeo-ung-lang\", branch = \"dev\", features = [\"number\"] }
",
        ),
    )?;
    io::print_log(stdout, "test pre-build")?;
    util::execute_command_stderr(
        stdout,
        &*format!(
            "cargo build --manifest-path={} --release --color {}",
            util::path_to_string(
                &hy_opt
                    .build_source
                    .as_ref()
                    .unwrap()
                    .join("hyeong-build/Cargo.toml")
            )?,
            if stdout.supports_color() {
                "always"
            } else {
                "none"
            }
        ),
    )?;
    io::print_note(stdout, "to uninstall, run `hyeong uninstall`")?;
    Ok(())
}

#[cfg_attr(tarpaulin, skip)]
pub fn uninstall_run(stdout: &mut StandardStream, hy_opt: HyeongOption) -> Result<(), Error> {
    io::print_log(stdout, "removing dir")?;
    fs::remove_dir_all(hy_opt.build_source.as_ref().unwrap())?;
    Ok(())
}
