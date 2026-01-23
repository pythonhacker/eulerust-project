use euler_lib::pandigital_product;

fn main() {
    let mut sum: u32 = pandigital_product(2,3);
    sum += pandigital_product(1, 4);

    println!("{}", sum);
}
