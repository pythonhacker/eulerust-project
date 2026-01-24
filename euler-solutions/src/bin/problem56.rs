// Maximum digital sum of powers
use num_bigint::{BigUint};
use euler_lib::{power, digit_to_vector_bigint};

fn main() {

    let mut num_vec: Vec<BigUint>;
    let mut num_pow;
    let mut max_sum: BigUint = 0u32.into();
    let (mut _max_a, mut _max_b) = (0u32, 0u32);
    
    for a in 2u32..100 {
        for b in 2..100 {
            num_pow = power(a.into(), b);
            num_vec = digit_to_vector_bigint(num_pow.to_owned());
            // Sum
            let sum: BigUint = num_vec.iter().fold(0u32.into(), |s, x| s+x);
            if sum > max_sum {
                // max_a = a;
				// max_b = b as u32;
				max_sum = sum;
            }
        }
    }

    println!("{}", max_sum);
}
