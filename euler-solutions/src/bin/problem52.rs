// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// Find the smallest positive integer, x, such that 2x, 3x, 4x, 5x, and 6x, contain the same digits.
use euler_lib::same_digits_multiples;

fn main() {

    println!("{}", same_digits_multiples(1000000));
}
