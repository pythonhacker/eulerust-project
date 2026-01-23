// Pentagonal numbers
use euler_lib::{generate_pentagonal_numbers, pentagon_min_diffsum};

fn main() {

    let numbers: Vec<u128> = generate_pentagonal_numbers(10000);
    let (_, _, min_diff) = pentagon_min_diffsum(numbers);
	println!("{}", min_diff);

}
