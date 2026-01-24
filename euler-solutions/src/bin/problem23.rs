// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// Find the sum of all the positive integers which cannot be written as the sum of two abundant numbers.

use euler_lib::{non_abundant_sum};

fn main() {
    println!("{}", non_abundant_sum(28123));    
}
