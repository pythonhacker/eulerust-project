// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use euler_lib::triplet_finder;

fn main() {

    let mut l_max: usize = 0;
    let mut n_max: u64 = 0;
    
    for x in 10u64..1000u64 {
        let triplets = triplet_finder(x);
        if triplets.len() > l_max {
            l_max = triplets.len();
            n_max = x;
            // println!("{} {}", l_max, n_max);
        }
    }
    
    println!("{}", n_max);
}
