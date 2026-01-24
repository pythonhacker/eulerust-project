// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// Coded triangle numbers
use euler_lib::{data_file, find_triangle_words};

fn main() {
	let fpath = data_file("problem42", "triangle_words.txt");	
    println!("{}", find_triangle_words(fpath.display().to_string()));
}
