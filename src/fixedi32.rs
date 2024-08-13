use core::{convert, ops};

#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Debug)]
pub struct FixedI32<const N: usize> {
    pub value: i32,
}

#[macro_export]
macro_rules! fixedI32 {
    ($whole_bits:expr, $value:literal) => {
        crate::fixedi32::FixedI32::<$whole_bits>::new(
            ($value as f32 * (1 << (32 - $whole_bits)) as f32) as i32,
        )
    };
}

impl<const N: usize> FixedI32<N> {
    pub const FRAC_SIZE: i32 = 32 - (N as i32);

    pub fn new(value: i32) -> Self {
        FixedI32 { value }
    }

    pub fn whole(self) -> i32 {
        self.value >> Self::FRAC_SIZE
    }

    pub fn frac(self) -> u32 {
        let mask = (1 << Self::FRAC_SIZE) - 1;
        (self.value & mask) as u32
    }
}

impl<const N: usize> convert::From<u32> for FixedI32<N> {
    fn from(whole: u32) -> Self {
        Self {
            value: (whole * (1 << (32 - N))) as i32,
        }
    }
}

impl<const N: usize> convert::From<i32> for FixedI32<N> {
    fn from(whole: i32) -> Self {
        Self {
            value: whole * (1 << (32 - N)),
        }
    }
}

impl<const N: usize> convert::Into<i32> for FixedI32<N> {
    fn into(self) -> i32 {
        self.value
    }
}

#[cfg(feature = "float")]
impl<const N: usize> convert::Into<f64> for FixedI32<N> {
    fn into(self) -> f64 {
        (self.value as f64) / 2.0f64.powf((32 - N) as f64)
    }
}

impl<const N: usize> ops::Add for FixedI32<N> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        FixedI32 {
            value: self.value + rhs.value,
        }
    }
}

impl<const N: usize> ops::AddAssign for FixedI32<N> {
    fn add_assign(&mut self, rhs: Self) {
        self.value += rhs.value;
    }
}

impl<const N: usize> ops::Sub for FixedI32<N> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        FixedI32 {
            value: self.value - rhs.value,
        }
    }
}

impl<const N: usize> ops::SubAssign for FixedI32<N> {
    fn sub_assign(&mut self, rhs: Self) {
        self.value -= rhs.value;
    }
}

impl<const N: usize> ops::Mul for FixedI32<N> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        FixedI32 {
            value: (((self.value as i64) * (rhs.value as i64)) >> Self::FRAC_SIZE) as i32,
        }
    }
}

impl<const N: usize> ops::MulAssign for FixedI32<N> {
    fn mul_assign(&mut self, rhs: Self) {
        self.value = (((self.value as i64) * (rhs.value as i64)) >> Self::FRAC_SIZE) as i32;
    }
}

impl<const N: usize> ops::Div for FixedI32<N> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        FixedI32 {
            value: (((self.value as i64) << (32 - N as i64)) / (rhs.value as i64)) as i32,
        }
    }
}

impl<const N: usize> ops::DivAssign for FixedI32<N> {
    fn div_assign(&mut self, rhs: Self) {
        self.value = (((self.value as i64) << (32 - N as i64)) / (rhs.value as i64)) as i32;
    }
}

#[cfg(feature = "defmt")]
impl<const N: usize> Format for FixedI32<N> {
    fn format(&self, fmt: Formatter) {
        // Whole part is wrong.....
        let whole = self.whole();
        let mut frac = self.frac() as u64;
        let frac_size = 32 - N;
        frac = frac * 10u64.pow(num_digits(frac_size)) / 2u64.pow(frac_size as u32);

        // TODO: this overflows really quickly, accuracy seems bad?

        write!(fmt, "{}.{}", whole, frac)
    }
}

#[cfg(feature = "fmt")]
impl<const N: usize> fmt::Display for FixedI32<N> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value: f64 = (*self).into();
        formatter.write_fmt(format_args!("{}", value))
    }
}
