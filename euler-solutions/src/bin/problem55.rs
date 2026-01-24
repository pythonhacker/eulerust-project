// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// Lychrel numbers
// Find count of Lychrel numbers < 10000

use euler_lib::{is_lychrel};

fn main() {
    let mut count = 0;
    let mut n:u32 = 1;
    let mut n_iter:u32;

    
    loop {
        // println!("Trying {}", n);
        n_iter = 0;
        
        if is_lychrel(n.into(), &mut n_iter) {
            // println!("IS LYCHREL => {}", n);
            count += 1;
        }

        n += 1;
        if n == 10000 { break; }
    }

    println!("{}", count);
}
