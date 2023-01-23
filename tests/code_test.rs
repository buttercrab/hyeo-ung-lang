#[cfg(test)]
mod code_test {
    use hyeong::core::code::Code;
    use hyeong::core::state::{State, UnOptState};
    use hyeong::core::{execute, optimize, parse};
    use hyeong::util::io;

    #[test]
    fn area_test01() {
        let t = format!("{:?}", parse::parse("하아앗..? 흑..".to_string())[0].get_area());
        assert_eq!("?__", t);
    }

    #[test]
    fn area_test02() {
        let t = format!(
            "{:?}",
            parse::parse("혀엉... 핫... 형..! 흑..".to_string())[2].get_area()
        );
        assert_eq!("!__", t);
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
        let t2 = (1, 0);
        assert_eq!(t1, t2);
    }

    #[test]
    fn un_opt_code_get_loc_test02() {
        let t1 = parse::parse("하아앗..💖 흑..".to_string())[0].get_location();
        let t2 = (1, 0);
        assert_eq!(t1, t2);
    }

    #[test]
    fn un_opt_state_test01() {
        let parsed = &parse::parse("형. 하앙...".to_string())[0];
        let mut ipt = io::CustomReader::new("".to_string());
        let mut out = io::CustomWriter::new(|_| Result::Ok(()));
        let mut err = io::CustomWriter::new(|_| Result::Ok(()));
        let mut state = UnOptState::new();
        state = execute::execute(&mut ipt, &mut out, &mut err, state, parsed).unwrap();
        let t1 = (state.get_all_stack_index()[0], state.stack_size());
        let t2 = (3, 1);
        assert_eq!(t1, t2);
    }

    #[test]
    fn un_opt_state_test02() {
        let parsed = &parse::parse("형. 하앙...".to_string())[0];
        let mut ipt = io::CustomReader::new("".to_string());
        let mut out = io::CustomWriter::new(|_| Result::Ok(()));
        let mut err = io::CustomWriter::new(|_| Result::Ok(()));
        let mut state = UnOptState::new();
        state = execute::execute(&mut ipt, &mut out, &mut err, state, parsed).unwrap();
        let t = format!("{:?}", state.get_all_code());
        assert_eq!(t, "[type: 0, cnt1: 1, cnt2: 1, area: \"_\"]")
    }

    #[test]
    fn un_opt_state_test03() {
        let parsed = &parse::parse("형. 하앙...".to_string())[0];
        let mut ipt = io::CustomReader::new("".to_string());
        let mut out = io::CustomWriter::new(|_| Result::Ok(()));
        let mut err = io::CustomWriter::new(|_| Result::Ok(()));
        let mut state = UnOptState::new();
        state = execute::execute(&mut ipt, &mut out, &mut err, state, parsed).unwrap();
        let t = state.get_all_point().len();
        assert_eq!(t, 0);
    }

    #[test]
    fn un_opt_state_test04() {
        let parsed = &parse::parse("형. 하앙...".to_string())[0];
        let mut ipt = io::CustomReader::new("".to_string());
        let mut out = io::CustomWriter::new(|_| Result::Ok(()));
        let mut err = io::CustomWriter::new(|_| Result::Ok(()));
        let mut state = UnOptState::new();
        state = execute::execute(&mut ipt, &mut out, &mut err, state, parsed).unwrap();
        let t = format!("{:?}", state);
        assert_eq!(t, "current stack: 3\nstack 3: [1]\n");
    }

    #[test]
    fn un_opt_state_test05() {
        let parsed = &parse::parse("형..💖 항..💖".to_string())[0];
        let mut ipt = io::CustomReader::new("".to_string());
        let mut out = io::CustomWriter::new(|_| Result::Ok(()));
        let mut err = io::CustomWriter::new(|_| Result::Ok(()));
        let mut state = UnOptState::new();
        state = execute::execute(&mut ipt, &mut out, &mut err, state, parsed).unwrap();
        let t = state.get_all_point()[0];
        assert_eq!(t, (37, 0));
    }

    #[test]
    fn un_opt_state_test06() {
        let parsed = &parse::parse("형..💖 항..💖".to_string())[0];
        let mut ipt = io::CustomReader::new("".to_string());
        let mut out = io::CustomWriter::new(|_| Result::Ok(()));
        let mut err = io::CustomWriter::new(|_| Result::Ok(()));
        let mut state = UnOptState::new();
        state = execute::execute(&mut ipt, &mut out, &mut err, state, parsed).unwrap();
        let t = format!("{:?}", state);
        assert_eq!(t, "current stack: 3\nstack 3: [2]\n");
    }

    #[test]
    fn opt_state_test01() {
        let un_opt_code = parse::parse("흑 하앙...".to_string());
        let mut ipt = io::CustomReader::new("1 2".to_string());
        let mut out = io::CustomWriter::new(|_| Result::Ok(()));
        let mut err = io::CustomWriter::new(|_| Result::Ok(()));
        let (mut opt_state, opt_code) = optimize::optimize(un_opt_code, 2).unwrap();
        opt_state = execute::execute(&mut ipt, &mut out, &mut err, opt_state, &opt_code[0]).unwrap();
        let t1 = (opt_state.get_all_stack_index()[1], opt_state.stack_size());
        let t2 = (1, 5);
        assert_eq!(t1, t2);
    }

    #[test]
    fn opt_state_test02() {
        let un_opt_code = parse::parse("흑 하앙...".to_string());
        let mut ipt = io::CustomReader::new("1 2".to_string());
        let mut out = io::CustomWriter::new(|_| Result::Ok(()));
        let mut err = io::CustomWriter::new(|_| Result::Ok(()));
        let (mut opt_state, opt_code) = optimize::optimize(un_opt_code, 2).unwrap();
        opt_state = execute::execute(&mut ipt, &mut out, &mut err, opt_state, &opt_code[0]).unwrap();
        let t = opt_state.get_all_code()[0].get_type();
        assert_eq!(t, 5);
    }

    #[test]
    fn opt_state_test03() {
        let un_opt_code = parse::parse("형..💖 항..💖".to_string());
        let mut ipt = io::CustomReader::new("".to_string());
        let mut out = io::CustomWriter::new(|_| Result::Ok(()));
        let mut err = io::CustomWriter::new(|_| Result::Ok(()));
        let (mut opt_state, opt_code) = optimize::optimize(un_opt_code, 1).unwrap();
        opt_state = execute::execute(&mut ipt, &mut out, &mut err, opt_state, &opt_code[0]).unwrap();
        let t1 = opt_state.get_all_point()[0];
        let t2 = (37, 0);
        assert_eq!(t1, t2);
    }
}
