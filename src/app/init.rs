use crate::util::error::Error;
use crate::util::option::HyeongOption;
use crate::util::{error, ext, io, option};
use clap::App;
use std::fs;
use termcolor::{StandardStream, WriteColor};

/// App for install
#[cfg(not(tarpaulin_include))]
pub fn install_app<'a>() -> App<'a> {
    App::new("install")
        .about("Install hyeong before build (need once)")
        .arg(option::build_path())
}

/// App for uninstall
#[cfg(not(tarpaulin_include))]
pub fn uninstall_app<'a>() -> App<'a> {
    App::new("uninstall")
        .about("Uninstall hyeong temporary build path")
        .arg(option::build_path())
}

/// Runner for install
///
/// 1. if dir is not empty -> Error
/// 2. create dir
/// 3. create Cargo.toml
/// 4. create main.rs
/// 5. pre-compile
#[cfg(not(tarpaulin_include))]
pub fn install_run(stdout: &mut StandardStream, hy_opt: &HyeongOption) -> Result<(), Error> {
    if hy_opt
        .build_path
        .as_ref()
        .unwrap()
        .join("hyeong-build/Cargo.toml")
        .exists()
    {
        return Err(Error::new(
            format!(
                "cannot install to {}",
                ext::path_to_string(hy_opt.build_path.as_ref().unwrap())?
            ),
            "already installed",
        ));
    }
    io::print_log(stdout, "making dir for building hyeong")?;
    fs::create_dir_all(&hy_opt.build_path.as_ref().unwrap().join("hyeong-build/src"))?;
    io::save_to_file(
        &hy_opt
            .build_path
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
            .build_path
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
hyeong = { git = \"https://github.com/buttercrab/hyeo-ung-lang\", features = [\"number\"], default-features = false }
",
        ),
    )?;
    io::print_log(stdout, "test pre-build")?;
    ext::execute_command_stderr(
        stdout,
        &*format!(
            "cargo build --manifest-path={} --release --color {}",
            ext::path_to_string(
                &hy_opt
                    .build_path
                    .as_ref()
                    .unwrap()
                    .join("hyeong-build/Cargo.toml")
            )?,
            if stdout.supports_color() {
                "always"
            } else {
                "never"
            }
        ),
    )?;
    io::print_note(stdout, "to uninstall, run `hyeong uninstall`")?;
    Ok(())
}

#[cfg(not(tarpaulin_include))]
pub fn uninstall_run(stdout: &mut StandardStream, hy_opt: &HyeongOption) -> Result<(), Error> {
    io::print_log(stdout, "removing dir")?;
    error::add_note(
        fs::remove_dir_all(hy_opt.build_path.as_ref().unwrap()),
        "already uninstalled",
    )?;
    Ok(())
}
