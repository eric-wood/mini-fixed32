use core::prelude::rust_2021::{derive, Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd};
use core::{convert, ops};

#[cfg(feature = "fmt")]
use std::fmt;

#[cfg(feature = "defmt")]
use defmt::{write, Format, Formatter};

/// A signed 32-bit fixed point number with N integer bits.
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Debug)]
pub struct FixedI32<const N: usize> {
    pub value: i32,
}

/// Create a new [FixedI32] from a floating point immediate at compile time.
///
/// ```
/// # use mini_fixed32::{fixedI32, FixedI32};
/// let pi = fixedI32!(16, 3.1415926);
/// ```
#[macro_export]
macro_rules! fixedI32 {
    ($whole_bits:expr, $value:literal) => {
        FixedI32::<$whole_bits>::new(($value as f32 * (1 << (32 - $whole_bits)) as f32) as i32)
    };
}

impl<const N: usize> FixedI32<N> {
    pub const FRAC_SIZE: i32 = 32 - (N as i32);

    /// Construct a new FixedI32 from an existin i32 fixed-point number.
    pub const fn new(value: i32) -> Self {
        FixedI32 { value }
    }

    /// Return the integer (whole) part of the number.
    pub fn whole(self) -> i32 {
        self.value >> Self::FRAC_SIZE
    }

    /// Return the fractional part of the number.
    pub fn frac(self) -> u32 {
        let mask = (1 << Self::FRAC_SIZE) - 1;
        (self.value & mask) as u32
    }

    pub fn abs(self) -> Self {
        FixedI32 {
            value: self.value.abs(),
        }
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

impl<const N: usize> ops::Neg for FixedI32<N> {
    type Output = Self;
    fn neg(self) -> Self {
        Self { value: -self.value }
    }
}

impl<const N: usize> ops::Rem for FixedI32<N> {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self {
        Self::new(self.value % rhs.value)
    }
}

#[cfg(feature = "defmt")]
impl<const N: usize> Format for FixedI32<N> {
    fn format(&self, fmt: Formatter) {
        write!(fmt, "f{}i{}", N, self.value)
    }
}

#[cfg(feature = "fmt")]
impl<const N: usize> fmt::Display for FixedI32<N> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value: f64 = (*self).into();
        formatter.write_fmt(format_args!("{}", value))
    }
}
