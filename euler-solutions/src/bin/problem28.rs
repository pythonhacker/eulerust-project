use euler_lib::sum_spiral_diagonal;

fn main() {
    let sum = (1..1002).step_by(2).map(|n| sum_spiral_diagonal(n as u64)).fold(0, |s,x| s+x);
    println!("{}",sum);	
}
