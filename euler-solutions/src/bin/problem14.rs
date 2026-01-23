// Longest collatz sequence
use euler_lib::collatz;

fn find_largest(n:i64) -> (i64, i64) {

    let (mut largest, mut cur_n) = (0,0);
    let mut cn;

    for x in 1..n+1 {
        cn = collatz(x);
        if cn > largest {
            largest = cn;
            cur_n = x;
        }
    }

    (largest, cur_n)
}

fn main() {
    println!("{:?}", find_largest(1000000));
}
