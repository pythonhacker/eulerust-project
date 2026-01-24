// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// Consecutive prime sum
//  Which prime, below one-million, can be written as the sum of the most consecutive primes?
use euler_lib::{max_consecutive_prime_sum};

fn main() {
    println!("{}", max_consecutive_prime_sum(1000000));
}
