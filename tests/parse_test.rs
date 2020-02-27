#[cfg(test)]
mod parse_test {
    use hyeo_ung_lang::parse::Command;

    fn basic_test(code: &str, res: &str) {
        let t = if let Ok(t) = Command::parse(code.parse().unwrap()) {
            assert_eq!(t.len(), 1);
            format!("{}", t[0])
        } else {
            "".to_string()
        };

        assert_eq!(t, res.to_string())
    }

    #[test]
    fn hangul_test01() { basic_test("형", "type: 0, cnt1: 1, cnt2: 0, area: _"); }

    #[test]
    fn hangul_test02() { basic_test("항", "type: 1, cnt1: 1, cnt2: 0, area: _"); }

    #[test]
    fn hangul_test03() { basic_test("핫", "type: 2, cnt1: 1, cnt2: 0, area: _"); }

    #[test]
    fn hangul_test04() { basic_test("흣", "type: 3, cnt1: 1, cnt2: 0, area: _"); }

    #[test]
    fn hangul_test05() { basic_test("흡", "type: 4, cnt1: 1, cnt2: 0, area: _"); }

    #[test]
    fn hangul_test06() { basic_test("흑", "type: 5, cnt1: 1, cnt2: 0, area: _"); }

    #[test]
    fn hangul_test07() { basic_test("혀엉", "type: 0, cnt1: 2, cnt2: 0, area: _"); }

    #[test]
    fn hangul_test08() { basic_test("하앙", "type: 1, cnt1: 2, cnt2: 0, area: _"); }

    #[test]
    fn hangul_test09() { basic_test("하앗", "type: 2, cnt1: 2, cnt2: 0, area: _"); }

    #[test]
    fn hangul_test10() { basic_test("흐읏", "type: 3, cnt1: 2, cnt2: 0, area: _"); }

    #[test]
    fn hangul_test11() { basic_test("흐읍", "type: 4, cnt1: 2, cnt2: 0, area: _"); }

    #[test]
    fn hangul_test12() { basic_test("흐윽", "type: 5, cnt1: 2, cnt2: 0, area: _"); }

    #[test]
    fn hangul_test13() { basic_test("흐...읍", "type: 4, cnt1: 2, cnt2: 0, area: _"); }

    #[test]
    fn hangul_test14() { basic_test("혀일이삼사오육앙앗읏읍엉", "type: 0, cnt1: 12, cnt2: 0, area: _"); }

    #[test]
    fn hangul_test15() { basic_test("혀일....이삼사오육앙♥앗?!읏♡읍...엉", "type: 0, cnt1: 12, cnt2: 0, area: _"); }

    #[test]
    fn hangul_test16() { basic_test("흐일이삼사 오육앙하앗읏읍엉", "type: 3, cnt1: 11, cnt2: 0, area: _"); }

    #[test]
    fn hangul_test17() { basic_test("하흐읏앗앙", "type: 2, cnt1: 4, cnt2: 0, area: _"); }

    #[test]
    fn hangul_test18() { basic_test("혀흐하윽", "type: 5, cnt1: 3, cnt2: 0, area: _"); }

    #[test]
    fn dot_test01() { basic_test("혀엉....", "type: 0, cnt1: 2, cnt2: 4, area: _"); }

    #[test]
    fn dot_test02() { basic_test("하앗. … ⋯ ⋮", "type: 2, cnt1: 2, cnt2: 10, area: _"); }

    #[test]
    fn dot_test03() { basic_test("혀읏......잠....하앙....혀엉. .....", "type: 0, cnt1: 7, cnt2: 6, area: _"); }

    #[test]
    fn dot_test04() { basic_test("흐읏.... 잠... 혀....", "type: 3, cnt1: 2, cnt2: 11, area: _"); }

    #[test]
    fn area_test01() { basic_test("하앗....♥♡!", "type: 2, cnt1: 2, cnt2: 4, area: !♥_"); }

    #[test]
    fn area_test02() { basic_test("하아앗.. . ? ♥ ! 💖", "type: 2, cnt1: 3, cnt2: 3, area: ?_!♥💖"); }

    #[test]
    fn area_test03() { basic_test("혀엉...♥?!♡", "type: 0, cnt1: 2, cnt2: 3, area: ?♥!_♡"); }
}