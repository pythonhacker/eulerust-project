// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use euler_lib::{amicable};

fn main() {

    let mut sum: i64 = 0;
    let mut amicables: Vec<i64> = vec![];

    for x in 2..10000 {
        if amicables.contains(&x) {
            continue;
        }

        sum += amicable(x, &mut amicables);
    }

    println!("{}", sum);
}
