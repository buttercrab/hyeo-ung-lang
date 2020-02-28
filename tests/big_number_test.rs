#[cfg(test)]
mod big_number_test {
    use hyeo_ung_lang::big_number::Num;

    #[test]
    fn add_test01() {
        let a = Num::new(1234);
        let b = Num::new(4321);
        let c = &a + &b;

        assert_eq!(c, Num::new(1234 + 4321))
    }

    #[test]
    fn add_test02() {
        let a = Num::from_vec(vec![4294967295, 4294967295]);
        let b = Num::from_vec(vec![4294967295, 4294967295]);
        let c = &a + &b;

        assert_eq!(c, Num::from_vec(vec![4294967294, 4294967295, 1]));
    }

    #[test]
    fn add_test03() {
        let a = Num::from_string("123456789123456789".to_string());
        let b = Num::from_string("987654321987654321".to_string());
        let c = &a + &b;

        assert_eq!(c, Num::from_string("1111111111111111110".to_string()));
    }

    #[test]
    fn sub_test01() {
        let a = Num::new(1234);
        let b = Num::new(4321);
        let c = &a - &b;

        assert_eq!(c, Num::new(1234 - 4321))
    }

    #[test]
    fn sub_test02() {
        let a = Num::from_vec(vec![0, 1]);
        let b = Num::from_vec(vec![4294967295]);
        let c = &a - &b;

        assert_eq!(c, Num::one());
    }

    #[test]
    fn sub_test03() {
        let a = Num::from_string("123456789123456789".to_string());
        let b = Num::from_string("987654321987654321".to_string());
        let c = &b - &a;

        assert_eq!(c, Num::from_string("864197532864197532".to_string()));
    }

    #[test]
    fn mul_test01() {
        let a = Num::new(1234);
        let b = Num::new(4321);
        let c = &a * &b;

        assert_eq!(c, Num::new(1234 * 4321));
    }

    #[test]
    fn mul_test02() {
        let a = Num::from_vec(vec![4294967295, 4294967295]);
        let b = Num::from_vec(vec![4294967295, 4294967295]);
        let c = &a * &b;

        assert_eq!(c, Num::from_vec(vec![1, 0, 4294967294, 4294967295]));
    }

    #[test]
    fn mul_test03() {
        let a = Num::from_string("238147328478237427348273487283478237482374".to_string());
        let b = Num::from_string("238478237492847735678364128937128937943".to_string());
        let c = &a * &b;

        assert_eq!(c, Num::from_string("56792955159120326139782764143873306061725717545674087612268430742682680802316682".to_string()));
    }

    #[test]
    fn div_test01() {
        let a = Num::new(4321);
        let b = Num::new(23);
        let c = &a / &b;

        assert_eq!(c, Num::new(187));
    }

    #[test]
    fn div_test02() {
        let a = Num::new(-1234);
        let b = Num::new(31);
        let c = &a / &b;

        // warn: this number is different from rust
        // this is valid when remainder is always pos
        assert_eq!(c, Num::new(-40));
    }

    #[test]
    fn from_string_test01() {
        let a = Num::from_string("1234".to_string());

        assert_eq!(a, Num::new(1234));
    }

    #[test]
    fn from_string_test02() {
        let a = Num::from_string("-1234".to_string());

        assert_eq!(a, Num::new(-1234));
    }

    #[test]
    fn from_string_test03() {
        let a = Num::from_string("12392389128391823928391123".to_string());

        assert_eq!(a, Num::from_vec(vec![3443848659, 3267458252, 671792]));
    }

    #[test]
    fn to_string_test01() {
        let a = Num::new(1234);

        assert_eq!(a.to_string(), "1234");
    }
}