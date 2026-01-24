// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// How many values of nCr where n <= 100 are greater than 1 million ?
use num_bigint::{BigUint};
use euler_lib::n_cr;

fn main() {

    let limit: BigUint = 1000000u32.into();
    let mut count: usize = 0;
    
    for n in 23..101 {
        // 100*99*98 < 1million so we need to go upto n-3 only
        for r in 2..n-3 {
            let n_cr = n_cr(n, r);
            if n_cr > limit {
                // println!("{}C{} => {} exceeds 1 million", n, r, n_cr);
                count += 1;
            }
        }
    }

    println!("{}", count);
}
