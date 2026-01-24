// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// Find the sum of all numbers, less than one million, which are palindromic in base 10 and base 2.
use euler_lib::{num_to_base2, string_is_palindrome};

fn main() {

    let mut sum:u64 = 0;
    
    for x in 1u64..1000000u64 {
        if string_is_palindrome(x.to_string()) && string_is_palindrome(num_to_base2(x)) {
            sum += x;
//            println!("{}", x);
        }
    }

    println!("{}", sum);
}
