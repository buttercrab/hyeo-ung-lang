#[cfg(test)]
mod code_test {
    use hyeong::{code, parse, io, execute, optimize};
    use hyeong::code::Code;
    use hyeong::code::State;

    #[test]
    fn area_test01() {
        let t = format!("{:?}",parse::parse("하아앗..? 흑..".to_string())[0].get_area());
        assert_eq!("?__", t);
    }

    #[test]
    fn area_test02() {
        let t = format!("{:?}",parse::parse("혀엉... 핫... 형..! 흑..".to_string())[2].get_area());
        assert_eq!("!__", t);
    }

    #[test]
    fn un_opt_code_to_string_test01() {
        let t = format!("{:?}",parse::parse("형".to_string())[0].to_string());
        assert_eq!("\"\\u{1b}[33m1:0\\u{1b}[0m 형_1_0 : _\"".to_string(), t);
    }

    #[test]
    fn un_opt_code_to_string_test02() {
        let t = format!("{:?}",parse::parse("하앗..".to_string())[0].to_string());
        assert_eq!("\"\\u{1b}[33m1:0\\u{1b}[0m 핫_2_2 : _\"".to_string(), t);
    }

    #[test]
    fn un_opt_code_get_raw_test01() {
        let t = parse::parse("형..".to_string())[0].get_raw();
        assert_eq!("형..", t);
    }

    #[test]
    fn un_opt_code_get_raw_test02() {
        let t = parse::parse("형..핫...".to_string())[0].get_raw();
        assert_eq!("형..", t);
    }

    #[test]
    fn un_opt_code_get_loc_test01() {
        let t1 = parse::parse("하앙..".to_string())[0].get_location();
        let t2 = (1,0);
        assert_eq!(t1,t2);
    }

    #[test]
    fn un_opt_code_get_loc_test02() {
        let t1 = parse::parse("하아앗..💖 흑..".to_string())[0].get_location();
        let t2 = (1,0);
        assert_eq!(t1,t2);
    }

    #[test]
    fn un_opt_state_test01() {
        let parsed = &parse::parse("형. 하앙...".to_string())[0];
        let mut ipt = io::CustomReader::new("".to_string());
        let mut out = io::CustomWriter::new(|_| Result::Ok(()));
        let mut err = io::CustomWriter::new(|_| Result::Ok(()));
        let mut state = code::UnOptState::new();
        state = execute::execute(&mut ipt, &mut out, &mut err, state, parsed);
        let t1 = (state.get_all_stack_index()[0], state.stack_size());
        let t2 = (3,1);
        assert_eq!(t1, t2);
    }

    #[test]
    fn un_opt_state_test02() {
        let parsed = &parse::parse("형. 하앙...".to_string())[0];
        let mut ipt = io::CustomReader::new("".to_string());
        let mut out = io::CustomWriter::new(|_| Result::Ok(()));
        let mut err = io::CustomWriter::new(|_| Result::Ok(()));
        let mut state = code::UnOptState::new();
        state = execute::execute(&mut ipt, &mut out, &mut err, state, parsed);
        let t = format!("{:?}",state.get_all_code());
        assert_eq!(t, "[type: 0, cnt1: 1, cnt2: 1, area: \"_\"]")
    }

    #[test]
    fn un_opt_state_test03() {
        let parsed = &parse::parse("형. 하앙...".to_string())[0];
        let mut ipt = io::CustomReader::new("".to_string());
        let mut out = io::CustomWriter::new(|_| Result::Ok(()));
        let mut err = io::CustomWriter::new(|_| Result::Ok(()));
        let mut state = code::UnOptState::new();
        state = execute::execute(&mut ipt, &mut out, &mut err, state, parsed);
        let t = state.get_all_point().len();
        assert_eq!(t, 0);
    }

    #[test]
    fn un_opt_state_test04() {
        let parsed = &parse::parse("형. 하앙...".to_string())[0];
        let mut ipt = io::CustomReader::new("".to_string());
        let mut out = io::CustomWriter::new(|_| Result::Ok(()));
        let mut err = io::CustomWriter::new(|_| Result::Ok(()));
        let mut state = code::UnOptState::new();
        state = execute::execute(&mut ipt, &mut out, &mut err, state, parsed);
        let t = format!("{:?}",state);
        assert_eq!(t, "current stack: 3\nstack 3: [1]\n");
    }
}