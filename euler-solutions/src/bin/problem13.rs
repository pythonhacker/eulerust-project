// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// Work out the first ten digits of the sum of the following one-hundred-digit numbers.

use euler_lib::{data_file, sum_numbers};

fn main() {
	let fpath = data_file("problem13", "numbers.txt");	
	let sum = sum_numbers(fpath.display().to_string());
    println!("{}", &sum.to_string()[0..10]);           	
}
