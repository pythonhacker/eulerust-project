use euler_lib::{amicable};

fn main() {

    let mut sum: i64 = 0;
    let mut amicables: Vec<i64> = vec![];

    for x in 2..10000 {
        if amicables.contains(&x) {
            continue;
        }

        sum += amicable(x, &mut amicables);
    }

    println!("{}", sum);
}
