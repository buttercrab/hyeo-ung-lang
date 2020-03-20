use crate::state::UnOptState;
use crate::{compile, io, optimize, option};
use clap::{App, ArgMatches};
use std::path::Path;
use std::process::Command;

#[cfg_attr(tarpaulin, skip)]
pub fn app<'a, 'b>() -> App<'a, 'b> {
    App::new("build")
        .about("Compiles hyeong code")
        .arg(option::build_source())
        .arg(option::input())
        .arg(option::optimize())
        .arg(option::output())
}

pub fn run(matches: &ArgMatches) {
    let file = matches.value_of("input").unwrap();
    let un_opt_code = io::read_file(file);
    let level_str = matches.value_of("optimize").unwrap();
    let level = io::handle_error(level_str.parse::<usize>());
    let output_file = match matches.value_of("output") {
        Some(v) => v.to_string(),
        None => {
            let v = file.split(".").collect::<Vec<_>>();
            v[..v.len() - 1].join(".")
        }
    };

    let source = if level >= 1 {
        let (state, opt_code) = optimize::optimize(un_opt_code, level);
        io::print_log("compiling to rust");
        compile::build_source(state, &opt_code, level)
    } else {
        let state = UnOptState::new();
        io::print_log("compiling to rust");
        compile::build_source(state, &un_opt_code, 0)
    };
    if !Path::new(&*io::get_build_path()).exists() {
        io::print_log("making temporary crate");
        io::execute_command_stderr(
            &*format!(
                "cargo new {} --color always --vcs none",
                io::get_build_path()
            ),
            &*format!(
                "cargo new {} --color always --vcs none",
                io::get_build_path()
            ),
        );
    }
    io::save_to_file(&*(io::get_build_path() + "/src/main.rs"), source);
    io::print_log("compiling rust code");
    io::execute_command_stderr(
        &*format!(
            "cargo build --manifest-path={}\\Cargo.toml --release --color always",
            io::get_build_path()
        ),
        &*format!(
            "cargo build --manifest-path={}/Cargo.toml --release --color always",
            io::get_build_path()
        ),
    );
    io::print_log("moving binary to current directory");
    if cfg!(target_os = "windows") {
        io::handle_error(
            Command::new("cmd")
                .arg("/C")
                .arg(format!(
                    "copy %USERPROFILE%\\.hyeong\\hyeong-build\\target\\release\\hyeong-build.exe {}.exe",
                    output_file
                ))
                .output()
        )
    } else {
        io::handle_error(
            Command::new("bash")
                .arg("-c")
                .arg(format!(
                    "cp \"$HOME\"/.hyeong/hyeong-build/target/release/hyeong-build {}",
                    output_file
                ))
                .output(),
        )
    };
    io::print_log("done!");
}
