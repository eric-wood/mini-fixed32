//!  A minimal 32-bit fixed point ([zero-cost](https://blog.rust-lang.org/2015/05/11/traits.html)) abstraction for microcontrollers.
//!
//! Intended for use with microcontrollers that lack an FPU where fractional values are still desired.
//! The much more feature-complete [fixed crate](https://crates.io/crates/fixed) takes a more complex approach that ballooned the size of my executables in this environment, when only a tiny subset of the features it offers were needed.
//!
//! ## Usage
//!
//! ```
//! # use mini_fixed32::{fixedU32, FixedU32};
//! // use the macro to convert from floating point at compile-time
//! let pi = fixedU32!(16, 3.1415926);
//! // load from a u32 representation
//! let two = FixedU32::<16>::new(131072);
//!
//! // wow stress-free arithmetic!
//! let two_pi = pi * two;
//! ```

#![cfg_attr(not(feature = "fmt"), no_std)]

mod fixedi32;
pub use fixedi32::FixedI32;

mod fixedu32;
pub use fixedu32::FixedU32;

#[cfg(test)]
mod test_fixedi32;

#[cfg(test)]
mod test_fixedu32;

mod util;
