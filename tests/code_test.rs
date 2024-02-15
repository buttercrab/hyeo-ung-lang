#[cfg(test)]
mod code_test {
    use hyeong::{
        hyeong::{
            area::HeartType,
            code::{Code, HangulType, OptCode, UnOptCode},
            execute::ExecutableState,
            optimize,
            parse::{self, Span},
            state::{OptState, State, UnOptState},
        },
        io,
    };
    use nom::error::ErrorKind;

    fn p<'a>(s: &'a str) -> Vec<UnOptCode<'a>> {
        parse::parse::<(Span, ErrorKind)>(s).unwrap()
    }

    #[test]
    fn area_test01() {
        let t = format!("{}", p("하아앗..? 흑..")[0].area());
        assert_eq!("[_]?[[_]]", t);
    }

    #[test]
    fn area_test02() {
        let t = format!("{}", p("혀엉... 핫... 형..! 흑..")[2].area());
        assert_eq!("[_]![_]", t);
    }

    #[test]
    fn un_opt_code_get_raw_test01() {
        let binding = p("형..");
        let t = binding[0].raw();
        assert_eq!("형..", t);
    }

    #[test]
    fn un_opt_code_get_raw_test02() {
        let binding = p("형..핫...");
        let t = binding[0].raw();
        assert_eq!("형..", t);
    }

    #[test]
    fn un_opt_code_get_loc_test01() {
        let line = p("하앙..")[0].start_span().location_line();
        let offset = p("하앙..")[0].start_span().location_offset();
        assert_eq!((1, 0), (line, offset));
    }

    #[test]
    fn un_opt_code_get_loc_test02() {
        let line = p("하아앗..💖 흑..")[0].start_span().location_line();
        let offset = p("하아앗..💖 흑..")[0].start_span().location_offset();
        assert_eq!((1, 0), (line, offset));
    }

    fn run_unopt<'a>(s: &'a str) -> UnOptState<'a> {
        let parsed = p(s)[0].clone();
        let mut ipt = io::CustomReader::new("".to_string());
        let mut out = io::CustomWriter::new(|_| Result::Ok(()));
        let mut err = io::CustomWriter::new(|_| Result::Ok(()));
        let mut state = UnOptState::new();
        state.execute(&mut ipt, &mut out, &mut err, parsed).unwrap();
        state
    }

    #[test]
    fn un_opt_state_test01() {
        let state = run_unopt("형. 하앙...");

        let t1 = (state.stack_indices()[0], state.stack_count());
        let t2 = (3, 1);

        assert_eq!(t1, t2);
    }

    #[test]
    fn un_opt_state_test02() {
        let state = run_unopt("형. 하앙...");
        let t = state.all_point().len();
        assert_eq!(t, 0);
    }

    #[test]
    fn un_opt_state_test03() {
        let state = run_unopt("형. 하앙...");
        let t = format!("{:?}", state);
        assert_eq!(t, "current stack: 3\nstack 3: [1]\n");
    }

    #[test]
    fn un_opt_state_test04() {
        let state = run_unopt("형..💖 항..💖");
        let t = state.all_point()[0];
        assert_eq!(t, ((2, HeartType::SparklingHeart), 0));
    }

    #[test]
    fn un_opt_state_test05() {
        let state = run_unopt("형..💖 항..💖");
        let t = format!("{:?}", state);
        assert_eq!(t, "current stack: 3\nstack 3: [2]\n");
    }

    fn run_opt(s: &str, level: u8, inp: &str) -> (OptState, Vec<OptCode>) {
        let parsed = p(s);
        let mut ipt = io::CustomReader::new(inp.to_string());
        let mut out = io::CustomWriter::new(|_| Result::Ok(()));
        let mut err = io::CustomWriter::new(|_| Result::Ok(()));

        let (mut state, code) = optimize::optimize(parsed, level).unwrap();
        state
            .execute(&mut ipt, &mut out, &mut err, code[0].clone())
            .unwrap();
        (state, code)
    }

    // #[test]
    // fn opt_state_test01() {
    //     let (opt_state, _) = run_opt("흑 하앙...", 2, "1 2");

    //     let t1 = (opt_state.stack_indices()[1], opt_state.stack_count());
    //     let t2 = (1, 5);
    //     assert_eq!(t1, t2);
    // }

    // #[test]
    // fn opt_state_test02() {
    //     let (opt_state, _) = run_opt("흑 하앙...", 2, "1 2");
    //     let t = opt_state.all_code()[0].hangul_type();
    //     assert_eq!(t, HangulType::Heuk);
    // }

    // #[test]
    // fn opt_state_test03() {
    //     let (opt_state, _) = run_opt("형..💖 항..💖", 1, "");
    //     assert_eq!(opt_state.all_point()[0], ((2, HeartType::SparklingHeart), 0));
    // }
}
