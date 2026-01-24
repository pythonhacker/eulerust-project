// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use euler_lib::{get_n_penta, get_n_hexa};

fn main() {

    let mut n: u64 = 286;
    let mut t: u64;
    let mut n1: f64;
    let mut n2: f64;
    
    loop {
        t = n*(n+1)/2;
        n1 = get_n_penta(t);
        n2 = get_n_hexa(t);

        // When we get an integer solution for this
        // that is the answer we want. This is the
        // fastest approach.
        if n1.trunc() == n1 && n2.trunc() == n2 {
            println!("{}",t);
            break;
        }

        n += 1;
    }

}
