use crate::app::init;
use crate::core::state::UnOptState;
use crate::core::{compile, optimize};
use crate::util::error::Error;
use crate::util::option::HyeongOption;
use crate::util::{io, option, util};
use clap::App;
use std::fs;
use termcolor::{StandardStream, WriteColor};

/// App for build
#[cfg_attr(tarpaulin, skip)]
pub fn app<'a, 'b>() -> App<'a, 'b> {
    App::new("build")
        .about("Compiles hyeong code")
        .arg(option::build_path())
        .arg(option::input())
        .arg(option::optimize())
        .arg(option::output())
}

/// Runner for build
///
/// 1. parse code
/// 2. optimize code
/// 3. install if build-dir is not set
/// 4. compile to binary
#[cfg_attr(tarpaulin, skip)]
pub fn run(stdout: &mut StandardStream, hy_opt: &HyeongOption) -> Result<(), Error> {
    // parse
    let un_opt_code = util::parse_file(stdout, &hy_opt.input.as_ref().unwrap())?;

    // optimize
    let rust_code = if hy_opt.optimize >= 1 {
        io::print_log(stdout, format!("optimizing to level {}", hy_opt.optimize))?;
        let (state, code) = optimize::optimize(un_opt_code, hy_opt.optimize)?;
        io::print_log(stdout, "compiling to rust")?;
        compile::build_source(state, &code, hy_opt.optimize)
    } else {
        let state = UnOptState::new();
        io::print_log(stdout, "compiling to rust")?;
        compile::build_source(state, &un_opt_code, hy_opt.optimize)
    };

    // install
    if !hy_opt
        .build_source
        .as_ref()
        .unwrap()
        .join("hyeong-build/Cargo.toml")
        .exists()
    {
        init::install_run(stdout, hy_opt)?;
    }

    // compile to binary
    io::save_to_file(
        &hy_opt
            .build_source
            .as_ref()
            .unwrap()
            .join("hyeong-build/src/main.rs"),
        rust_code,
    )?;
    io::print_log(stdout, "compiling rust code")?;
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
                "never"
            }
        ),
    )?;

    // move
    io::print_log(stdout, "moving binary to current directory")?;
    fs::copy(
        hy_opt
            .build_source
            .as_ref()
            .unwrap()
            .join(if cfg!(windows) {
                "hyeong-build/target/release/hyeong-build.exe"
            } else {
                "hyeong-build/target/release/hyeong-build"
            }),
        hy_opt.output.as_ref().unwrap(),
    )?;

    Ok(())
}
