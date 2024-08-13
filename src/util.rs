/// Dumb branching method for determining the number of digits for a
/// given power of 2 without bringing in a logarithm function.
pub fn num_digits(power: usize) -> u32 {
    match power {
        power if power < 4 => 1,
        power if power < 7 => 2,
        power if power < 10 => 3,
        power if power < 14 => 4,
        power if power < 17 => 5,
        power if power < 20 => 6,
        power if power < 24 => 7,
        power if power < 27 => 8,
        power if power < 30 => 9,
        power if power < 34 => 10,
        _ => 0,
    }
}
