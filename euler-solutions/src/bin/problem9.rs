use euler_lib::pythagorean_triplet_finder;

fn main() {

    let t = pythagorean_triplet_finder(1000);
    println!("{}", t.0*t.1*t.2);
}
