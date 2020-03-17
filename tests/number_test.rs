#[cfg(test)]
mod number_test {
    use hyeong::number::Num;
    use std::cmp::Ordering;

    #[test]
    fn compare_test01() {
        let a = Num::nan();
        let b = Num::one();

        assert!(matches!(a.partial_cmp(&b), Option::None));
    }

    #[test]
    fn compare_test02() {
        let a = Num::one();
        let b = Num::one();

        assert!(matches!(a.partial_cmp(&b), Option::Some(Ordering::Equal)));
    }

    #[test]
    fn compare_test03() {
        let a = Num::zero();
        let b = Num::one();

        assert!(matches!(a.partial_cmp(&b), Option::Some(Ordering::Less)));
    }

    #[test]
    fn add_test01() {
        let a = Num::nan();
        let b = Num::one();

        assert_eq!(Num::nan(), &a + &b);
    }

    #[test]
    fn mult_test01() {
        let a = Num::one();
        let b = Num::nan();

        assert_eq!(Num::nan(), &a * &b);
    }

    #[test]
    fn flip_test01() {
        let mut a = Num::new(-10, 3);
        a.flip();

        assert_eq!(Num::new(-3, 10), a);
    }
}
