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

    // #[test]
    // fn mul_test01() {
    //     let a = Num::from_string("111111111".to_string());
    //     let b = Num::from_string("111111111".to_string());
    //
    //     assert_eq!(&a * &b, Num::from_string("12345678987654321".to_string()));
    // }

    // #[test]
    // fn from_string_test01() {
    //     let a = Num::from_string("1234".to_string());
    //
    //     assert_eq!(a, Num::new(1234));
    // }
}