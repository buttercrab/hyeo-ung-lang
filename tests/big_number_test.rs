#[cfg(test)]
mod big_number_test {
    use std::cmp::Ordering;

    use hyeong::big_number::{BigNum, Error};

    #[test]
    fn add_test01() {
        let a = BigNum::new(1234);
        let b = BigNum::new(4321);
        let c = &a + &b;

        assert_eq!(BigNum::new(1234 + 4321), c)
    }

    #[test]
    fn add_test02() {
        let a = BigNum::from_vec(vec![4294967295, 4294967295]);
        let b = BigNum::from_vec(vec![4294967295, 4294967295]);
        let c = &a + &b;

        assert_eq!(BigNum::from_vec(vec![4294967294, 4294967295, 1]), c);
    }

    #[test]
    fn add_test03() {
        let a = BigNum::from_string("123456789123456789".to_string()).unwrap();
        let b = BigNum::from_string("987654321987654321".to_string()).unwrap();
        let c = &a + &b;

        assert_eq!(BigNum::from_string("1111111111111111110".to_string()).unwrap(), c);
    }

    #[test]
    fn add_test04() {
        let a = BigNum::from_vec(vec![1]);
        let b = BigNum::from_vec(vec![4294967295, 4294967295]);
        let c = &a + &b;

        assert_eq!(BigNum::from_vec(vec![0, 0, 1]), c);
    }

    #[test]
    fn add_test05() {
        let a = BigNum::new(-1234);
        let b = BigNum::new(4321);
        let c = &a + &b;

        assert_eq!(BigNum::new(-1234 + 4321), c);
    }

    #[test]
    fn add_test06() {
        let a = BigNum::new(-1234);
        let b = BigNum::new(-4321);
        let c = &a + &b;

        assert_eq!(BigNum::new(-1234 - 4321), c);
    }

    #[test]
    fn sub_test01() {
        let a = BigNum::new(1234);
        let b = BigNum::new(4321);
        let c = &a - &b;

        assert_eq!(BigNum::new(1234 - 4321), c)
    }

    #[test]
    fn sub_test02() {
        let a = BigNum::from_vec(vec![0, 1]);
        let b = BigNum::from_vec(vec![4294967295]);
        let c = &a - &b;

        assert_eq!(BigNum::one(), c);
    }

    #[test]
    fn sub_test03() {
        let a = BigNum::from_string("123456789123456789".to_string()).unwrap();
        let b = BigNum::from_string("987654321987654321".to_string()).unwrap();
        let c = &b - &a;

        assert_eq!(BigNum::from_string("864197532864197532".to_string()).unwrap(), c);
    }

    #[test]
    fn sub_test04() {
        let a = BigNum::from_vec(vec![0, 0, 1]);
        let b = BigNum::from_vec(vec![1]);
        let c = &a - &b;

        assert_eq!(BigNum::from_vec(vec![4294967295, 4294967295]), c);
    }

    #[test]
    fn sub_test05() {
        let a = BigNum::new(-1234);
        let b = BigNum::new(-4321);
        let c = &a - &b;

        assert_eq!(BigNum::new(-1234 + 4321), c);
    }

    #[test]
    fn sub_test06() {
        let a = BigNum::new(-1234);
        let b = BigNum::new(4321);
        let c = &a - &b;

        assert_eq!(BigNum::new(-1234 - 4321), c);
    }

    #[test]
    fn mul_test01() {
        let a = BigNum::new(1234);
        let b = BigNum::new(4321);
        let c = &a * &b;

        assert_eq!(BigNum::new(1234 * 4321), c);
    }

    #[test]
    fn mul_test02() {
        let a = BigNum::from_vec(vec![4294967295, 4294967295]);
        let b = BigNum::from_vec(vec![4294967295, 4294967295]);
        let c = &a * &b;

        assert_eq!(BigNum::from_vec(vec![1, 0, 4294967294, 4294967295]), c);
    }

    #[test]
    fn mul_test03() {
        let a = BigNum::from_string("238147328478237427348273487283478237482374".to_string()).unwrap();
        let b = BigNum::from_string("238478237492847735678364128937128937943".to_string()).unwrap();
        let c = &a * &b;

        assert_eq!(BigNum::from_string("56792955159120326139782764143873306061725717545674087612268430742682680802316682".to_string()).unwrap(), c);
    }

    #[test]
    fn div_test01() {
        let a = BigNum::new(4321);
        let b = BigNum::new(23);
        let c = &a / &b;

        assert_eq!(BigNum::new(187), c);
    }

    #[test]
    fn div_test02() {
        let a = BigNum::new(-1234);
        let b = BigNum::new(31);
        let c = &a / &b;

        // warn: this number is different from rust
        // this is valid when remainder is always pos
        assert_eq!(BigNum::new(-1234 / 31), c);
    }

    #[test]
    fn from_string_test01() {
        let a = BigNum::from_string("1234".to_string()).unwrap();

        assert_eq!(BigNum::new(1234), a);
    }

    #[test]
    fn from_string_test02() {
        let a = BigNum::from_string("-1234".to_string()).unwrap();

        assert_eq!(BigNum::new(-1234), a);
    }

    #[test]
    fn from_string_test03() {
        let a = BigNum::from_string("12392389128391823928391123".to_string()).unwrap();

        assert_eq!(BigNum::from_vec(vec![3443848659, 3267458252, 671792]), a);
    }

    #[test]
    fn from_string_test04() {
        let a = BigNum::from_string("1234!".to_string());

        if let Result::Err(Error::ParseError) = a {
            assert!(true);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn from_string_test05() {
        let a = BigNum::from_string_base("1234".to_string(), 100);

        if let Result::Err(Error::BaseSizeError(100)) = a {
            assert!(true);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn to_string_test01() {
        let a = BigNum::new(1234);

        assert_eq!("1234", a.to_string());
    }

    #[test]
    fn to_string_test02() {
        let a = BigNum::new(-1234);

        assert_eq!("-1234", a.to_string());
    }

    #[test]
    fn to_string_test03() {
        let a = BigNum::new(1234);
        let b = a.to_string_base(100);

        if let Result::Err(Error::BaseSizeError(100)) = b {
            assert!(true);
        }
    }

    #[test]
    fn equal_test01() {
        let a = BigNum::zero();
        let b = BigNum::zero();

        assert_eq!(a, b);
    }

    #[test]
    fn compare_test01() {
        let a = BigNum::new(1234);
        let b = BigNum::new(-4321);

        if let Option::Some(Ordering::Greater) = a.partial_cmp(&b) {
            assert!(true);
        }
    }

    #[test]
    fn compare_test02() {
        let a = BigNum::new(-1234);
        let b = BigNum::new(4321);

        if let Option::Some(Ordering::Less) = a.partial_cmp(&b) {
            assert!(true);
        }
    }

    #[test]
    fn compare_test03() {
        let a = BigNum::new(-1234);
        let b = BigNum::new(-4321);

        if let Option::Some(Ordering::Greater) = a.partial_cmp(&b) {
            assert!(true);
        }
    }
}