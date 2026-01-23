use euler_lib::max_recurrence_fraction;

fn main() {
	let (max_n, _max_length) = max_recurrence_fraction(1000);
	println!("{}", max_n);
}
