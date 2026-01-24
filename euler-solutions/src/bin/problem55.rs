// Lychrel numbers
// Find count of Lychrel numbers < 10000

use euler_lib::{is_lychrel};

fn main() {
    let mut count = 0;
    let mut n:u32 = 1;
    let mut n_iter:u32;

    
    loop {
        // println!("Trying {}", n);
        n_iter = 0;
        
        if is_lychrel(n.into(), &mut n_iter) {
            // println!("IS LYCHREL => {}", n);
            count += 1;
        }

        n += 1;
        if n == 10000 { break; }
    }

    println!("{}", count);
}
