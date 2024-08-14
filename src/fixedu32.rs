use core::prelude::rust_2021::{derive, Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd};
use core::{convert, ops};

#[cfg(feature = "fmt")]
use std::fmt;

#[cfg(feature = "defmt")]
use crate::util::num_digits;

#[cfg(feature = "defmt")]
use defmt::{write, Format, Formatter};

#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Debug)]
pub struct FixedU32<const N: usize> {
    pub value: u32,
}

#[macro_export]
macro_rules! fixedU32 {
    ($whole_bits:expr, $value:literal) => {
        FixedU32::<$whole_bits>::new(($value as f32 * (1 << (32 - $whole_bits)) as f32) as u32)
    };
}

impl<const N: usize> FixedU32<N> {
    pub const FRAC_SIZE: u32 = 32 - (N as u32);

    pub const fn new(value: u32) -> Self {
        FixedU32 { value }
    }

    pub fn whole(self) -> u32 {
        self.value >> Self::FRAC_SIZE
    }

    pub fn frac(self) -> u32 {
        let mask = (1 << Self::FRAC_SIZE) - 1;
        self.value & mask
    }
}

impl<const N: usize> convert::From<u32> for FixedU32<N> {
    fn from(whole: u32) -> Self {
        Self {
            value: whole * (1 << (32 - N)),
        }
    }
}

impl<const N: usize> convert::From<i32> for FixedU32<N> {
    fn from(whole: i32) -> Self {
        Self {
            value: (whole * (1 << (32 - N))) as u32,
        }
    }
}

impl<const N: usize> convert::Into<u32> for FixedU32<N> {
    fn into(self) -> u32 {
        self.value
    }
}

#[cfg(feature = "float")]
impl<const N: usize> convert::Into<f64> for FixedU32<N> {
    fn into(self) -> f64 {
        (self.value as f64) / 2.0f64.powf((32 - N) as f64)
    }
}

impl<const N: usize> ops::Add for FixedU32<N> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        FixedU32 {
            value: self.value + rhs.value,
        }
    }
}

impl<const N: usize> ops::AddAssign for FixedU32<N> {
    fn add_assign(&mut self, rhs: Self) {
        self.value += rhs.value;
    }
}

impl<const N: usize> ops::Sub for FixedU32<N> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        FixedU32 {
            value: self.value - rhs.value,
        }
    }
}

impl<const N: usize> ops::SubAssign for FixedU32<N> {
    fn sub_assign(&mut self, rhs: Self) {
        self.value -= rhs.value;
    }
}

impl<const N: usize> ops::Mul for FixedU32<N> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        FixedU32 {
            value: (((self.value as u64) * (rhs.value as u64)) >> Self::FRAC_SIZE) as u32,
        }
    }
}

impl<const N: usize> ops::MulAssign for FixedU32<N> {
    fn mul_assign(&mut self, rhs: Self) {
        self.value = (((self.value as u64) * (rhs.value as u64)) >> Self::FRAC_SIZE) as u32;
    }
}

impl<const N: usize> ops::Div for FixedU32<N> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        FixedU32 {
            value: (((self.value as u64) << (32 - N as u64)) / (rhs.value as u64)) as u32,
        }
    }
}

impl<const N: usize> ops::DivAssign for FixedU32<N> {
    fn div_assign(&mut self, rhs: Self) {
        self.value = (((self.value as u64) << (32 - N as u64)) / (rhs.value as u64)) as u32
    }
}

#[cfg(feature = "defmt")]
impl<const N: usize> Format for FixedU32<N> {
    fn format(&self, fmt: Formatter) {
        let whole = self.whole();
        let mut frac = self.frac() as u64;
        let frac_size = 32 - N;
        frac = frac * 10u64.pow(num_digits(frac_size)) / 2u64.pow(frac_size as u32);

        write!(fmt, "{}.{}", whole, frac)
    }
}

#[cfg(feature = "fmt")]
impl<const N: usize> fmt::Display for FixedU32<N> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value: f64 = (*self).into();
        formatter.write_fmt(format_args!("{}", value))
    }
}
