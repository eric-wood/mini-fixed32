# mini-fixed32

A minimal 32-bit fixed point ([zero-cost](https://blog.rust-lang.org/2015/05/11/traits.html)) abstraction for microcontrollers.

## Why?

I commonly work with Cortex M0 cores that lack an FPU and find myself needing to represent fractional values. The much more feature-complete [fixed crate](https://crates.io/crates/fixed) takes a more complex approach that ballooned the size of my executables in this environment, and I typically only needed a tiny subset of the features it offers.

By representing fixed point numbers with these structs, we sidestep common mathematical pitfalls and can lean on Rust's type system to enforce safety and consistency.

## Features

- Arithmetic operations!
- Comparison!
- Formatting! (with experimental [defmt](http://defmt.ferrous-systems.com) support)
- Conversion!
- Floating point immediates!

## Usage

For the most part you just construct your numbers and go about your day normally:

```rs
// use the macro to convert from floating point at compile-time
let pi = fixedU32!(16, 3.1415926);
// load from a u32 representation
let two = FixedU32::<16>::new(131072);

// wow stress-free arithmetic!
let two_pi = pi * two;

// displays as floating point when printed
println!("{}", two_pi);
```

### Cargo Features

Because this is primarily meant for use in a `no-std` environment, some functionality is gated behind features:

- `defmt`: experimental support for defmt formatting
- `float`: floating point conversion functions (e.g. `from` and `into`)
- `fmt`: formatting via `std::format` (enables `float`)

## Caveats

- Math beyond basic arithmetic is considered out of scope; look to crates like [cordic](https://crates.io/crates/cordic) for that
- When debugging with defmt, it does its best to attempt to display it as a float on-device, which is occasionally inaccurate given the constraints. This may get addressed later.
