// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use euler_lib::{data_file, TriangleGrid};

pub fn main() {
	let mut mytriangle = TriangleGrid::new(0, 0, 0);
	let fpath = data_file("problem18", "triangle.txt");
    mytriangle.read(fpath.display().to_string());
	
	println!("{}", mytriangle.max_sum(1, 0));
}
