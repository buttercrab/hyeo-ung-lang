#[cfg(test)]
mod execute_test {
    use hyeong::{
        hyeong::{
            execute::ExecutableState,
            parse::{self, Span},
            state::UnOptState,
        },
        io,
    };
    use nom::error::ErrorKind;

    fn helper_function(code: &str, stdin: &str, stdout: &str, stderr: &str) {
        let parsed = parse::parse::<(Span, ErrorKind)>(code).unwrap();
        let mut ipt = io::CustomReader::new(stdin.to_string());
        let mut out = io::CustomWriter::new(|_| Result::Ok(()));
        let mut err = io::CustomWriter::new(|_| Result::Ok(()));
        let mut state = UnOptState::new();

        for c in parsed {
            // state = execute::execute(&mut ipt, &mut out, &mut err, state, &c).unwrap();
            state.execute(&mut ipt, &mut out, &mut err, c).unwrap();
        }

        assert_eq!(stdout.to_string(), out.to_string().unwrap());
        assert_eq!(stderr.to_string(), err.to_string().unwrap());
    }

    #[test]
    fn execute_test01() {
        helper_function("혀어어어어어어엉......핫.", "", "0", "");
    }

    #[test]
    fn execute_test02() {
        helper_function("혀어어어어어어어엉........ 핫. 혀엉..... 흑... 하앗... 흐윽... 형.  하앙.혀엉.... 하앙... 흐윽... 항. 항. 형... 하앙. 흐으윽... 형... 흡... 혀엉..하아아앗. 혀엉.. 흡... 흐읍... 형.. 하앗. 하아앙... 형... 하앙... 흐윽...혀어어엉.. 하앙. 항. 형... 하앙. 혀엉.... 하앙. 흑... 항. 형... 흡  하앗.", "", "Hello, world!", "");
    }

    #[test]
    fn execute_test03() {
        helper_function("혀어어어엉.. 흐으으윽... 하앗... 형.. 하앙. 하앗... 형. 혀어어엉.... 하아앙. 혀어엉... 흐윽.... 형.. 하앙.... 하앗.... 흐윽.... 핫. 혀엉.... 하앙. 혀어어엉.. 혀엉.. 하앗. 혀어어어엉.. 형. 하앙.... 흐윽.... 하앗. 혀엉..... 흐으윽... 하앗... 형. 하아앙. 혀엉..... 흐으윽... 하앗... 혀어어어어어엉. 하아앙.", "", "fuck you", "");
    }

    #[test]
    fn execute_test04() {
        helper_function(
            "혀어어어어어어엉......핫.. 혀어어어어어어어엉........ 핫. 혀어어어어어어어엉......... 핫..",
            "",
            "H",
            "0Q",
        );
    }

    #[test]
    fn execute_test05() {
        helper_function("형. 흣..", "", "", "\u{1}");
    }
}
