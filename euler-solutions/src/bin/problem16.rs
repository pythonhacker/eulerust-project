// What is the sum of the digits of the number 2**1000 ?

// This is a one-liner in Python and gets way too complicated in Rust.
// It is a big surprise that Rust being such a high performant language
// does not have support for Big integers in its own standard library.
// Forced to use 2 external crates.

use euler_lib::power_sum;

fn main() {
    println!("{}", power_sum(2, 1000));
}
