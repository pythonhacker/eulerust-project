// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use euler_lib::pythagorean_triplet_finder;

fn main() {

    let t = pythagorean_triplet_finder(1000);
    println!("{}", t.0*t.1*t.2);
}
