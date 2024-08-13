#![no_std]

mod fixedi32;
pub use fixedi32::FixedI32;

mod fixedu32;
pub use fixedu32::FixedU32;

#[cfg(test)]
mod test_fixedi32;

#[cfg(test)]
mod test_fixedu32;

mod util;
