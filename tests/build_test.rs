#[cfg(test)]
mod io_test {
    use hyeong::{build, optimize, io, parse};
    use hyeong::state::{State, UnOptState};
    use std::io::Write;
    use std::path::Path;
    use std::{process, env};

    fn helper_function1(code: &str, level: usize) -> String {
        let un_opt_code = parse::parse(code.to_string());
        let un_opt_state = UnOptState::new();
        let mut out = io::CustomWriter::new(|_| Result::Ok(()));
        let mut err = io::CustomWriter::new(|_| Result::Ok(()));
        if level >= 1 {
            let (mut opt_state, opt_code) = optimize::optimize(un_opt_code, level);
            if !opt_state.get_stack(1).is_empty() {
                io::handle_error(out.flush());
                opt_state.get_stack(1).clear();
            }
            if !opt_state.get_stack(2).is_empty() {
                io::handle_error(err.flush());
                opt_state.get_stack(2).clear();
            }
            build::build_source(opt_state, &opt_code, level)
        } else {
            build::build_source(un_opt_state, &un_opt_code, level)
        }    
    }

    fn helper_function2(source: &str, output_file: &str, build_path: &str) {
        if !Path::new(&*build_path.to_string()).exists() {
            io::print_log("making temporary crate");
            io::execute_command_stderr(
                &*format!(
                    "cargo new {} --color always --vcs none",
                    build_path.to_string()
                ),
                &*format!(
                    "cargo new {} --color always --vcs none",
                    build_path.to_string()
                ),
            );
        }
        io::save_to_file(&*(build_path.to_string() + "/src/main.rs"), source.to_string());
        io::execute_command_stderr(
            &*format!(
                "cargo build --manifest-path={}\\Cargo.toml --release --color always",
                build_path.to_string()
            ),
            &*format!(
                "cargo build --manifest-path={}/Cargo.toml --release --color always",
                build_path.to_string()
            ),
        );
        if cfg!(target_os = "windows") {
            io::handle_error(process::Command::new("cmd").arg("/C").arg(format!(
                "copy {}\\target\\release\\hyeong-build.exe {}.exe",
                build_path.to_string(),output_file.to_string()
            )).output())
        } else {
            io::handle_error(
                process::Command::new("bash")
                    .arg("-c")
                    .arg(format!(
                        "cp {}/target/release/hyeong-build {}",
                        build_path.to_string(),output_file.to_string()
                    ))
                    .output(),
            )
        };
    }

    #[test]
    fn build_test01() {
        let source = helper_function1("형.. 흣.", 0);
        let output_file = "examples\\build_test01\\build_test01";
        let build_path = if cfg!(target_os = "windows") {
            env::var("USERPROFILE").unwrap() + "\\.hyeong\\build_test01\\hyeong-build"
        } else {
            env::var("HOME").unwrap() + "/.hyeong/build_test01/hyeong-build"
        };
        helper_function2(&source, &output_file, &build_path);
        let output = if cfg!(target_os = "windows") {
            io::handle_error(process::Command::new("cmd").arg("/C").arg("examples\\build_test01\\build_test01.exe").output())
        } else {
            io::handle_error(
                process::Command::new("bash")
                    .arg("-c")
                    .arg("examples/build_test01/build_test01")
                    .output(),
            )
        };
        let mut outstr = "".to_string();
        for i in output.stdout {
            outstr.push_str(&(i as u8 as char).to_string());
        }
        assert_eq!("2",outstr);
    }

    #[test]
    fn build_test02() {
        let source = helper_function1("혀어어어어어어어엉........ 핫. 혀엉..... 흑... 하앗... 흐윽... 형.  하앙.혀엉.... 하앙... 흐윽... 항. 항. 형... 하앙. 흐으윽... 형... 흡... 혀엉..하아아앗. 혀엉.. 흡... 흐읍... 형.. 하앗. 하아앙... 형... 하앙... 흐윽...혀어어엉.. 하앙. 항. 형... 하앙. 혀엉.... 하앙. 흑... 항. 형... 흡  하앗. 
        혀엉..... 흑. 흣", 1);
        let output_file = "examples\\build_test02\\build_test02";
        let build_path = if cfg!(target_os = "windows") {
            env::var("USERPROFILE").unwrap() + "\\.hyeong\\build_test02\\hyeong-build"
        } else {
            env::var("HOME").unwrap() + "/.hyeong/build_test02/hyeong-build"
        };
        helper_function2(&source, &output_file, &build_path);
        let output = if cfg!(target_os = "windows") {
            io::handle_error(process::Command::new("cmd").arg("/C").arg("examples\\build_test02\\build_test02.exe").output())
        } else {
            io::handle_error(
                process::Command::new("bash")
                    .arg("-c")
                    .arg("examples/build_test02/build_test02")
                    .output(),
            )
        };
        let mut outstr = "".to_string();
        for i in output.stdout {
            outstr.push_str(&(i as u8 as char).to_string());
        }
        assert_eq!("Hello, world!\n",outstr);
    }

    #[test]
    fn build_test03() {
        let source = helper_function1("형 흣........💕 흣.... 형. 하앙... 흣. 흑... 흐읏....!💕", 1);
        let output_file = "examples\\build_test03\\build_test03";
        let build_path = if cfg!(target_os = "windows") {
            env::var("USERPROFILE").unwrap() + "\\.hyeong\\build_test03\\hyeong-build"
        } else {
            env::var("HOME").unwrap() + "/.hyeong/build_test03/hyeong-build"
        };
        helper_function2(&source, &output_file, &build_path);
        let output = if cfg!(target_os = "windows") {
            io::handle_error(process::Command::new("cmd").arg("/C").arg("examples\\build_test03\\build_test03.exe").output())
        } else {
            io::handle_error(
                process::Command::new("bash")
                    .arg("-c")
                    .arg("examples/build_test03/build_test03")
                    .output(),
            )
        };
        let mut outstr = "".to_string();
        for i in output.stdout {
            outstr.push_str(&(i as u8 as char).to_string());
        }
        assert_eq!("12345678",outstr);
    }

    #[test]
    fn build_test04() {
        let source = helper_function1("형. 형.. 형. 흑...💘 항.... 하앙... 항...♡ 흑...💘 ! 흣...흑.", 0);
        let output_file = "examples\\build_test04\\build_test04";
        let build_path = if cfg!(target_os = "windows") {
            env::var("USERPROFILE").unwrap() + "\\.hyeong\\build_test04\\hyeong-build"
        } else {
            env::var("HOME").unwrap() + "/.hyeong/build_test04/hyeong-build"
        };
        helper_function2(&source, &output_file, &build_path);
        let output = if cfg!(target_os = "windows") {
            io::handle_error(process::Command::new("cmd").arg("/C").arg("examples\\build_test04\\build_test04.exe").output())
        } else {
            io::handle_error(
                process::Command::new("bash")
                    .arg("-c")
                    .arg("examples/build_test04/build_test04")
                    .output(),
            )
        };
        let mut outstr = "".to_string();
        for i in output.stdout {
            outstr.push_str(&(i as u8 as char).to_string());
        }
        assert_eq!("4",outstr);
    }

}