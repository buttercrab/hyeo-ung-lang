#[cfg(test)]
mod big_number_test {
    use hyeo_ung_lang::big_number::Num;

    #[test]
    fn add_test01() {
        let a = Num::new(1234);
        let b = Num::new(4321);

        assert_eq!(&a + &b, Num::new(1234 + 4321))
    }

    // #[test]
    // fn add_test02() {
    //     let a = Num::from_vec(vec![4294967295, 4294967295]);
    //     let b = Num::from_vec(vec![4294967295, 4294967295]);
    //
    //     assert_eq!(&a + &b, Num::from_vec(vec![1, 4294967295, 4294967294]));
    // }

    #[test]
    fn sub_test() {
        let a = Num::new(1234);
        let b = Num::new(4321);

        assert_eq!(&a - &b, Num::new(1234 - 4321))
    }

    // #[test]
    // fn mul_test01() {
    //     let a = Num::from_string("111111111".to_string());
    //     let b = Num::from_string("111111111".to_string());
    //
    //     assert_eq!(&a * &b, Num::from_string("12345678987654321".to_string()));
    // }
}