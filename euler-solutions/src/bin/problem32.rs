// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use euler_lib::pandigital_product;

fn main() {
    let mut sum: u32 = pandigital_product(2,3);
    sum += pandigital_product(1, 4);

    println!("{}", sum);
}
