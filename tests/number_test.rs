#[cfg(test)]
mod number_test {
    use std::cmp::Ordering;

    use hyeong::number::Num;

    #[test]
    fn compare_test01() {
        let a = Num::nan();
        let b = Num::one();

        if let Option::None = a.partial_cmp(&b) {
            assert!(true);
        }
    }

    #[test]
    fn compare_test02() {
        let a = Num::one();
        let b = Num::one();

        if let Option::Some(Ordering::Equal) = a.partial_cmp(&b) {
            assert!(true);
        }
    }

    #[test]
    fn compare_test03() {
        let a = Num::zero();
        let b = Num::one();

        if let Option::Some(Ordering::Less) = a.partial_cmp(&b) {
            assert!(true);
        }
    }
}