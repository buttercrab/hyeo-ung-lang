#[cfg(test)]
mod optimize_test {
    use std::io::Write;

    use hyeong::{
        hyeong::{
            execute::ExecutableState,
            optimize,
            parse::{self, Span},
            state::State,
        },
        io,
    };
    use nom::error::ErrorKind;

    fn helper_function(code: &str, stdin: &str, stdout: &str, stderr: &str, level: u8) {
        let un_opt_code = parse::parse::<(Span, ErrorKind)>(code).unwrap();
        let mut ipt = io::CustomReader::new(stdin.to_string());
        let mut out = io::CustomWriter::new(|_| Result::Ok(()));
        let mut err = io::CustomWriter::new(|_| Result::Ok(()));
        let mut out_str = String::from("");
        let mut err_str = String::from("");
        let (mut opt_state, opt_code) = optimize::optimize(un_opt_code, level).unwrap();

        if !opt_state.stack(1).is_empty() {
            for num in opt_state.stack(1).iter() {
                out_str.push_str(&*format!("{}", num.floor().to_int() as u8 as char));
            }
            out.flush().unwrap();
            opt_state.stack(1).clear();
        }

        if !opt_state.stack(2).is_empty() {
            for num in opt_state.stack(2).iter() {
                err_str.push_str(&*format!("{}", num.floor().to_int() as u8 as char));
            }
            err.flush().unwrap();
            opt_state.stack(2).clear();
        }

        for c in opt_code {
            opt_state.execute(&mut ipt, &mut out, &mut err, c).unwrap();
        }
        out_str.push_str(&out.to_string().unwrap());
        err_str.push_str(&err.to_string().unwrap());
        assert_eq!(out_str, stdout.to_string());
        assert_eq!(err_str, stderr.to_string());
    }

    #[test]
    fn optimize_test01() {
        helper_function("혀어어어어어어엉......핫.", "", "0", "", 2);
    }

    #[test]
    fn optimize_test02() {
        helper_function("혀어어어어어어어엉........ 핫. 혀엉..... 흑... 하앗... 흐윽... 형.  하앙.혀엉.... 하앙... 흐윽... 항. 항. 형... 하앙. 흐으윽... 형... 흡... 혀엉..하아아앗. 혀엉.. 흡... 흐읍... 형.. 하앗. 하아앙... 형... 하앙... 흐윽...혀어어엉.. 하앙. 항. 형... 하앙. 혀엉.... 하앙. 흑... 항. 형... 흡  하앗.", "", "Hello, world!", "", 2);
    }

    #[test]
    fn optimize_test03() {
        helper_function("혀어어어엉.. 흐으으윽... 하앗... 형.. 하앙. 하앗... 형. 혀어어엉.... 하아앙. 혀어엉... 흐윽.... 형.. 하앙.... 하앗.... 흐윽.... 핫. 혀엉.... 하앙. 혀어어엉.. 혀엉.. 하앗. 혀어어어엉.. 형. 하앙.... 흐윽.... 하앗. 혀엉..... 흐으윽... 하앗... 형. 하아앙. 혀엉..... 흐으윽... 하앗... 혀어어어어어엉. 하아앙.", "", "fuck you", "", 2);
    }

    #[test]
    fn optimize_test04() {
        helper_function(
            "혀어어어어어어엉......핫.. 혀어어어어어어어엉........ 핫. 혀어어어어어어어엉......... 핫..",
            "",
            "H",
            "0Q",
            2,
        );
    }

    #[test]
    fn optimize_test05() {
        helper_function(
            "형 흣........💕 흣.... 형. 하앙... 흣. 흑... 흐읏....!💕",
            "",
            "12345678",
            "",
            2,
        );
    }

    #[test]
    fn optimize_test06() {
        helper_function("형. 흣..", "", "", "1", 2);
    }

    #[test]
    fn optimize_test07() {
        helper_function("형 형 흣
        흑💘!💘 흑...! 하앙... 혀엉... .. 하앗... 흑!?! 흑... 혀어어 어어어 어엉... ... 흣... . 하앙... 흑 혀엉... .. 흣... . 하앙 흑...! 흑?💘?
        흑...! 항... . 혀엉... .. 흡... . 하앗...
        흑!?! 흑...!
        형 형
        흑💕!💕 흑...! 하앙... 혀엉... .. 하앗... 흑!?! 흑... 혀어어 어어어 어엉... ... 흣... . 하앙... 흑 혀엉... .. 흣... . 하앙 흑...! 흑?💕?
        흑...! 항... . 혀엉... .. 흡... . 하앗...
        흐읏.", "1111 1234", "2345", "", 2);
    }

    #[test]
    fn optimize_test08() {
        helper_function(
            "형. 형.. 형. 흑...💘 항.... 하앙... 항...♡ 흑...💘 ! 흣...흑.",
            "",
            "4",
            "",
            2,
        );
    }

    #[test]
    fn optimize_test09() {
        helper_function("형. 흣... 흑 항.", "", "1", "", 2);
    }

    #[test]
    fn optimize_test10() {
        helper_function("형. 흣... 흑 핫.", "", "1", "", 2);
    }

    #[test]
    fn optimize_test11() {
        helper_function("형. 흑 흣.", "", "1", "", 2);
    }

    #[test]
    fn optimize_test12() {
        helper_function("형. 흣... 흑 흡.", "", "1", "", 2);
    }

    #[test]
    fn optimize_test13() {
        helper_function("형. 흣... 흑 흑.", "", "1", "", 2);
    }
}
