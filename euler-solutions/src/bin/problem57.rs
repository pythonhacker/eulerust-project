// Square root convergents
// In the first one-thousand expansions of sqrt(2) as a continued fraction, how many fractions
// contain a numerator with more digits than the denominator?

use num_bigint::{BigUint};
use euler_lib::continued_fraction;
use std::collections::HashMap;

fn main() {
    let mut cache: HashMap<u32, (BigUint, BigUint)> = HashMap::new();
    let mut count: usize = 0;
        
    for i in 1..1001 {
        let fract = continued_fraction(i, &mut cache);
        let num_s = fract.0.to_string();
        let denom_s = fract.1.to_string();

        if num_s.len() > denom_s.len() {
            count += 1;
            // println!("Iteration #{} is a hit", i);
            // println!("{} {}", fract.0, fract.1);
        }
    }

    println!("{}", count);

}
