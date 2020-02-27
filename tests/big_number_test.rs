#[cfg(test)]
mod big_number_test {
    use hyeo_ung_lang::big_number::Num;

    #[test]
    fn add_test1() {
        let a = Num::new(1234);
        let b = Num::new(4321);

        assert_eq!(&a + &b, Num::new(1234 + 4321))
    }

    #[test]
    fn add_test2() {
        let a = Num::from_vec(vec![4294967295, 4294967295]);
        let b = Num::from_vec(vec![4294967295, 4294967295]);

        assert_eq!(&a + &b, Num::from_vec(vec![1, 4294967295, 4294967294]));
    }

    #[test]
    fn sub_test() {
        let a = Num::new(1234);
        let b = Num::new(4321);

        assert_eq!(&a - &b, Num::new(1234 - 4321))
    }
}