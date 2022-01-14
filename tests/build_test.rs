#[cfg(test)]
mod build_test {
    use hyeong::app::init;
    use hyeong::core::state::UnOptState;
    use hyeong::core::{compile, optimize, parse};
    use hyeong::util::option::HyeongOption;
    use hyeong::util::{io, util};
    use std::env;
    use std::path::PathBuf;
    use std::process::Command;
    use termcolor::{ColorChoice, StandardStream};

    fn helper_function(name: &str, code: &str, level: u8) -> String {
        let un_opt_code = parse::parse(code.to_string());
        let un_opt_state = UnOptState::new();
        let source = if level >= 1 {
            let (opt_state, opt_code) = optimize::optimize(un_opt_code, level).unwrap();
            compile::build_source(opt_state, &opt_code, level)
        } else {
            compile::build_source(un_opt_state, &un_opt_code, level)
        };

        let mut s = StandardStream::stdout(ColorChoice::Auto);
        let mut p = if cfg!(target_os = "windows") {
            PathBuf::from(&env::var("USERPROFILE").unwrap().replace('\\', "/"))
        } else {
            PathBuf::from(&env::var("HOME").unwrap())
        };
        p.push(format!(".hyeong/test/{}", name));
        if !p.join("hyeong-build/Cargo.toml").exists() {
            init::install_run(&mut s, &HyeongOption::new().build_path(p.clone())).unwrap();
        }
        p.push("hyeong-build");

        io::save_to_file(&p.join("src/main.rs"), source).unwrap();

        util::execute_command_stdout(
            &mut s,
            &*format!(
                "cargo build --manifest-path={} --release",
                util::path_to_string(&p.join("Cargo.toml")).unwrap()
            ),
        )
        .unwrap();

        String::from_utf8(
            Command::new("bash")
                .arg("-c")
                .arg(
                    &*util::path_to_string(&p.join(if cfg!(target_os = "windows") {
                        "target/release/hyeong-build.exe"
                    } else {
                        "target/release/hyeong-build"
                    }))
                    .unwrap(),
                )
                .output()
                .unwrap()
                .stdout,
        )
        .unwrap()
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
