// Consecutive prime sum
//  Which prime, below one-million, can be written as the sum of the most consecutive primes?
use euler_lib::{max_consecutive_prime_sum};

fn main() {
    println!("{}", max_consecutive_prime_sum(1000000));
}
