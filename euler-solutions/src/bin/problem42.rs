// Coded triangle numbers
use euler_lib::{data_file, find_triangle_words};

fn main() {
	let fpath = data_file("problem42", "triangle_words.txt");	
    println!("{}", find_triangle_words(fpath.display().to_string()));
}
