#[cfg(test)]
mod test_fixedi32 {
    use crate::{fixedI32, FixedI32};

    #[test]
    fn test_whole() {
        let num = fixedI32!(16, 1.234);
        assert_eq!(num.whole(), 1);
    }

    #[test]
    fn test_frac() {
        let num = fixedI32!(16, 1.234);
        let frac = fixedI32!(16, 0.234);
        assert_eq!(num.frac(), frac.value as u32);
    }

    #[test]
    fn test_add() {
        let mut a = fixedI32!(16, 3);
        let b = fixedI32!(16, -1.5);
        let answer = fixedI32!(16, 1.5);
        assert_eq!(a + b, answer);

        a += b;
        assert_eq!(a, answer);
    }

    #[test]
    fn test_sub() {
        let mut a = fixedI32!(16, 3);
        let b = fixedI32!(16, -1.5);
        let answer = fixedI32!(16, 4.5);
        assert_eq!(a - b, answer);

        a -= b;
        assert_eq!(a, answer);
    }

    #[test]
    fn test_mul() {
        let mut a = fixedI32!(16, 1.5);
        let b = fixedI32!(16, -2);
        let answer = fixedI32!(16, -3);
        assert_eq!(a * b, answer);

        a *= b;
        assert_eq!(a, answer);
    }

    #[test]
    fn test_div() {
        let mut a = fixedI32!(16, 3);
        let b = fixedI32!(16, -2);
        let answer = fixedI32!(16, -1.5);
        assert_eq!(a / b, answer);

        a /= b;
        assert_eq!(a, answer);
    }

    #[test]
    fn test_neg() {
        let a = fixedI32!(16, 1.5);
        let b = fixedI32!(16, -1.5);
        assert_eq!(-a, b);
        assert_eq!(-b, a);
    }

    #[test]
    fn test_abs() {
        let a = fixedI32!(16, -1.5);
        let answer = fixedI32!(16, 1.5);
        assert_eq!(a.abs(), answer);
    }
}
