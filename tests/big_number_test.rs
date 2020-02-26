#[cfg(test)]
mod big_number_test {
    use hyeo_ung_lang::big_number::Num;

    #[test]
    fn add_test() {
        let a = Num::new(1234);
        let b = Num::new(4321);

        assert_eq!(Num::add(&a, &b), Num::new(1234 + 4321))
    }

    #[test]
    fn sub_test() {
        let a = Num::new(1234);
        let b = Num::new(4321);

        assert_eq!(Num::sub(&a, &b), Num::new(1234 - 4321))
    }
}