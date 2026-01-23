// Work out the first ten digits of the sum of the following one-hundred-digit numbers.

use euler_lib::{data_file, sum_numbers};

fn main() {
	let fpath = data_file("problem13", "numbers.txt");	
	let sum = sum_numbers(fpath.display().to_string());
    println!("{}", &sum.to_string()[0..10]);           	
}
