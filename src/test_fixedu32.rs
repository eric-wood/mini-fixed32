#[cfg(test)]
mod test_fixedu32 {
    use crate::{fixedU32, FixedU32};

    #[test]
    fn test_whole() {
        let num = fixedU32!(16, 1.234);
        assert_eq!(num.whole(), 1);
    }

    #[test]
    fn test_frac() {
        let num = fixedU32!(16, 1.234);
        let frac = fixedU32!(16, 0.234);
        assert_eq!(num.frac(), frac.value as u32);
    }

    #[test]
    fn test_add() {
        let mut a = fixedU32!(16, 3);
        let b = fixedU32!(16, 1.5);
        let answer = fixedU32!(16, 4.5);
        assert_eq!(a + b, answer);

        a += b;
        assert_eq!(a, answer);
    }

    #[test]
    fn test_sub() {
        let mut a = fixedU32!(16, 3);
        let b = fixedU32!(16, 1.5);
        let answer = fixedU32!(16, 1.5);
        assert_eq!(a - b, answer);

        a -= b;
        assert_eq!(a, answer);
    }

    #[test]
    fn test_mul() {
        let mut a = fixedU32!(16, 1.5);
        let b = fixedU32!(16, 2);
        let answer = fixedU32!(16, 3);
        assert_eq!(a * b, answer);

        a *= b;
        assert_eq!(a, answer);
    }

    fn test_div() {
        let mut a = fixedU32!(16, 3);
        let b = fixedU32!(16, 2);
        let answer = fixedU32!(16, 1.5);
        assert_eq!(a / b, answer);

        a /= b;
        assert_eq!(a, answer);
    }
}
