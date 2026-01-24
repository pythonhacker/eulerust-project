// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use euler_lib::max_recurrence_fraction;

fn main() {
	let (max_n, _max_length) = max_recurrence_fraction(1000);
	println!("{}", max_n);
}
