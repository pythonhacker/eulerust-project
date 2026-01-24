// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// Distinct prime factors

use euler_lib::{consecutive_prime_factors};

fn main () {
    let _factors = consecutive_prime_factors(4);
    println!("{}", _factors[0]);
//    for n in _factors.iter() {
//        println!("Prime factors of {} => {:?}", *n, get_prime_factors_u32(*n));
//    }
}

