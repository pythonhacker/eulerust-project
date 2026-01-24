// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// Highly divisible triangular number
// See the problem at https://projecteuler.net/problem=12

use std::collections::HashMap;
use euler_lib::{get_triangle_number, get_divisors_triangle};

// Get first triangle number with "n" divisors
pub fn get_first_with_n_divisors(n:i64) -> i64 {

    let mut number:i64 = 100;
    let mut n_divisors;
    let mut divisors: HashMap<i64, i64> = HashMap::new();
    
    loop {
        n_divisors = get_divisors_triangle(number, &mut divisors);
        if n_divisors >= n {
            break;
        }

        number += 1;
    }

    get_triangle_number(number)
}


fn main() {
    println!("{}", get_first_with_n_divisors(500));
}

