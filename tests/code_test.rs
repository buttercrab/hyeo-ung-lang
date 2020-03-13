#[cfg(test)]
mod code_test {
    use hyeong::{code, parse, io, optimize};

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
}