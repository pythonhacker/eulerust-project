use euler_lib::{fibonacci_thousand_digits};

fn main() {
	let (idx, _num) = fibonacci_thousand_digits();
    println!("{}", idx);
}
