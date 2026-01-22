// A palindromic number reads the same both ways. The largest palindrome made
// from the product of two 2-digit numbers is
// 9009 = 91 x 99.
// Find the largest palindrome made from the product of two 3-digit numbers.
use euler_lib::{is_palindrome, largest_palindrome};

fn main() {
    println!("{}",is_palindrome(100002));
    println!("{}", largest_palindrome(1000,100));
}
