use euler_lib::{square_sum, sum_squares};

// Find the difference between the sum of the squares of the first one
// hundred natural numbers and the square of the sum.
fn main(){
	println!("{}", square_sum(100) - sum_squares(100));
}
