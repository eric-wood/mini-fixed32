#! python3

# Process incoming logs from defmt and replace fixed point numbers with their floating point counterparts
# The types defined in this crate override defmt to print their values in the format: `f{q}{type}{value}`
# which we can intercept from STDIN and replace with a converted float version.
# This script is meant to be piped to from the defmt logger, e.g.
# `cargo run | defmt_fixed.py`

import re
import fileinput

pattern = re.compile("f([0-9]+)([ui])([0-9]+)")

for line in fileinput.input():
    result = line
    for match in pattern.findall(result):
        q, sign, value = match
        q = int(q)
        value = int(value)

        float = value / (2 ** (32 - q))
        replacement = re.compile(f"f{q}{sign}{value}")
        result = replacement.sub(str(float), result)
    print(result)
