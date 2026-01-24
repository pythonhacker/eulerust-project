// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use euler_lib::{circles, is_prime};

fn main() {

    let mut count = 0;
    
    for i in 2u64..1000000u64 {
        // Do evaluation in one go using map and fold
        let all_prime: bool = circles(i).iter().map(|d| is_prime(*d)).fold(true, |p, x| p & x);
        if all_prime {
            // println!("{}", i);
            count += 1;
        }
    }

    println!("{}", count);

}
