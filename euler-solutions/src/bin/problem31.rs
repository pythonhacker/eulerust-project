// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// Coin sums
// How many different ways can 2 pounds be made using any number of coins?
use euler_lib::coin_sums;

fn main() {
    println!("{}",coin_sums(200, 0));
}
