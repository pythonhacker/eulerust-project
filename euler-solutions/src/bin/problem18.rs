use euler_lib::{data_file, TriangleGrid};

pub fn main() {
	let mut mytriangle = TriangleGrid::new(0, 0, 0);
	let fpath = data_file("problem18", "triangle.txt");
    mytriangle.read(fpath.display().to_string());
	
	println!("{}", mytriangle.max_sum(1, 0));
}
