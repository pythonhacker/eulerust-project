// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use euler_lib::sum_spiral_diagonal;

fn main() {
    let sum = (1..1002).step_by(2).map(|n| sum_spiral_diagonal(n as u64)).fold(0, |s,x| s+x);
    println!("{}",sum);	
}
