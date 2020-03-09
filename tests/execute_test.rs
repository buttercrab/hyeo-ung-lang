#[cfg(test)]
mod execute_test {
    use hyeong::code::UnOptState;
    use hyeong::{execute, io, parse};

    fn helper_function(code: &str, stdout: &str, stderr: &str) -> bool {
        let parsed = parse::parse(code.to_string());
        let mut out = io::CustomWriter::new(|_| Result::Ok(()));
        let mut err = io::CustomWriter::new(|_| Result::Ok(()));
        let mut state = UnOptState::new();

        for c in parsed {
            state = execute::execute(&mut out, &mut err, state, &c);
        }

        stdout.to_string() == out.to_string() && stderr.to_string() == err.to_string()
    }

    #[test]
    fn execute_test01() {
        assert!(helper_function("혀어어어어어어엉......핫.", "0", ""));
    }

    #[test]
    fn execute_test02() {
        assert!(helper_function("혀어어어어어어어엉........ 핫. 혀엉..... 흑... 하앗... 흐윽... 형.  하앙.혀엉.... 하앙... 흐윽... 항. 항. 형... 하앙. 흐으윽... 형... 흡... 혀엉..하아아앗. 혀엉.. 흡... 흐읍... 형.. 하앗. 하아앙... 형... 하앙... 흐윽...혀어어엉.. 하앙. 항. 형... 하앙. 혀엉.... 하앙. 흑... 항. 형... 흡  하앗.", "Hello, world!", ""));
    }

    #[test]
    fn execute_test03() {
        assert!(helper_function("혀어어어엉.. 흐으으윽... 하앗... 형.. 하앙. 하앗... 형. 혀어어엉.... 하아앙. 혀어엉... 흐윽.... 형.. 하앙.... 하앗.... 흐윽.... 핫. 혀엉.... 하앙. 혀어어엉.. 혀엉.. 하앗. 혀어어어엉.. 형. 하앙.... 흐윽.... 하앗. 혀엉..... 흐으윽... 하앗... 형. 하아앙. 혀엉..... 흐으윽... 하앗... 혀어어어어어엉. 하아앙.", "fuck you", ""));
    }

    #[test]
    fn execute_test04() {
        assert!(helper_function("혀어어어어어어엉......핫.. 혀어어어어어어어엉........ 핫. 혀어어어어어어어엉......... 핫..", "H", "0Q"));
    }

    #[test]
    fn execute_test05() {
        assert!(helper_function(
            "형 흣........💕 흣.... 형. 하앙... 흣. 흑... 흐읏....!💕",
            "12345678",
            ""
        ));
    }

    #[test]
    fn execute_test06() {
        assert!(helper_function("형. 흑. 항..", "", ""));
    }

    #[test]
    fn execute_test07() {
        assert!(helper_function("형. 흑.. 항.", "", ""));
    }

    #[test]
    fn execute_test08() {
        assert!(helper_function("형. 흣..", "", "1"));
    }
}
