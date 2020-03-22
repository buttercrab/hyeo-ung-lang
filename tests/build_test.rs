/*#[cfg(test)]
mod build_test {
    use hyeong::core::state::UnOptState;
    use hyeong::core::{compile, optimize, parse};
    use hyeong::util::io;
    use std::env;
    use std::process::Command;

    fn helper_function(name: &str, code: &str, level: usize) -> String {
        let un_opt_code = parse::parse(code.to_string());
        let un_opt_state = UnOptState::new();
        let source = if level >= 1 {
            let (opt_state, opt_code) = optimize::optimize(un_opt_code, level);
            compile::build_source(opt_state, &opt_code, level)
        } else {
            compile::build_source(un_opt_state, &un_opt_code, level)
        };

        let build_path = &*if cfg!(target_os = "windows") {
            format!(
                "{}\\.core\\test\\{}\\core-build",
                env::var("USERPROFILE").unwrap(),
                name
            )
        } else {
            format!(
                "{}/.core/test/{}/core-build",
                env::var("HOME").unwrap(),
                name
            )
        };

        io::execute_command(
            &*format!("rmdir /S {}", build_path),
            &*format!("rm -rf {}", build_path),
        );

        io::execute_command(
            &*format!("cargo new {} --vcs none", build_path),
            &*format!("cargo new {} --vcs none", build_path),
        );
        io::execute_command(
            &*format!("COPY /y src\\number.rs {}\\src\\number.rs", build_path),
            &*format!("cp src/number.rs {}/src/", build_path),
        );
        io::execute_command(
            &*format!(
                "COPY /y src\\big_number.rs {}\\src\\big_number.rs",
                build_path
            ),
            &*format!("cp src/big_number.rs {}/src/", build_path),
        );
        io::execute_command(
            &*format!(
                "echo pub mod big_number;pub mod number; > {}\\src\\lib.rs",
                build_path
            ),
            &*format!(
                "printf \"pub mod big_number;\npub mod number;\" > {}/src/lib.rs",
                build_path
            ),
        );

        io::save_to_file(
            (build_path.to_string()
                + if cfg!(target_os = "windows") {
                    "\\src\\main.rs"
                } else {
                    "/src/main.rs"
                })
            .as_str(),
            source,
        );

        io::execute_command(
            &*format!(
                "cargo build --manifest-path={}\\Cargo.toml --release",
                build_path
            ),
            &*format!(
                "cargo build --manifest-path={}/Cargo.toml --release",
                build_path
            ),
        );

        io::handle(String::from_utf8(
            io::handle(if cfg!(target_os = "windows") {
                Command::new("cmd")
                    .arg("/C")
                    .arg(&*format!("{}\\target\\release\\core-build.exe", build_path))
                    .output()
            } else {
                Command::new("bash")
                    .arg("-c")
                    .arg(&*format!("{}/target/release/core-build", build_path))
                    .output()
            })
            .stdout,
        ))
    }

    #[test]
    fn build_test01() {
        assert_eq!("2", helper_function("1", "형.. 흣.", 0));
    }

    #[test]
    fn build_test02() {
        assert_eq!(
            "Hello, world!\n",
            helper_function(
                "2",
                "혀어어어어어어어엉........ 핫. 혀엉..... 흑... 하앗... 흐윽... 형.\
                       하앙.혀엉.... 하앙... 흐윽... 항. 항. 형... 하앙. 흐으윽... 형... 흡... \
                       혀엉..하아아앗. 혀엉.. 흡... 흐읍... 형.. 하앗. 하아앙... 형... 하앙... \
                       흐윽...혀어어엉.. 하앙. 항. 형... 하앙. 혀엉.... 하앙. 흑... 항. 형... \
                       흡  하앗. 혀엉..... 흑. 흣",
                2,
            )
        );
    }

    #[test]
    fn build_test03() {
        assert_eq!(
            "12345678",
            helper_function(
                "3",
                "형 흣........💕 흣.... 형. 하앙... 흣. 흑... 흐읏....!💕",
                1,
            )
        );
    }

    #[test]
    fn build_test04() {
        assert_eq!(
            "4",
            helper_function(
                "4",
                "형. 형.. 형. 흑...💘 항.... 하앙... 항...♡ 흑...💘 ! 흣...흑.",
                0,
            )
        );
    }
}
*/
