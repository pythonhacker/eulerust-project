// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// Find the sum of all numbers which are equal to the sum of the factorial of their digits.
use euler_lib::{curious};

fn main() {

    let mut sum: u64 = 0;
    
    for i in 11u64..1000000u64 {
        // No point in doing factorials which are multiples of 10
        if i % 10 == 0 { continue; }
        if curious (i) {
            // println!("{}", i);
            sum += i;
        }
    }

    println!("{}",sum);

}
    
