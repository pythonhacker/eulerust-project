// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use euler_lib::{square_sum, sum_squares};

// Find the difference between the sum of the squares of the first one
// hundred natural numbers and the square of the sum.
fn main(){
	println!("{}", square_sum(100) - sum_squares(100));
}
