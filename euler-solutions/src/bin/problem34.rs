// Find the sum of all numbers which are equal to the sum of the factorial of their digits.
use euler_lib::{curious};

fn main() {

    let mut sum: u64 = 0;
    
    for i in 11u64..1000000u64 {
        // No point in doing factorials which are multiples of 10
        if i % 10 == 0 { continue; }
        if curious (i) {
            // println!("{}", i);
            sum += i;
        }
    }

    println!("{}",sum);

}
    
