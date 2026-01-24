// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// Find the sum of the only eleven primes that are both truncatable from left to right and right to left.
use euler_lib::{is_prime, is_truncatable_prime};

fn main() {

    let mut sum: u64 = 0;
    
    for x in 11..10000000 {
        if !is_prime(x) {continue;}
        if is_truncatable_prime(x) {
            sum += x;
        }
    }

    println!("{}", sum);
}
